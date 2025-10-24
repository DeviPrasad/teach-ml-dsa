use getrandom;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use crate::err::MlDsaError;
use crate::ntt::{mod_q, ntt, ntt_add, ntt_inverse, ntt_multiply};
use crate::params::{D, ETA, K, L, N, Q};

// https://github.com/post-quantum-cryptography/KAT/blob/main/MLDSA/kat_MLDSA_44_hedged_raw.rsp
pub fn key_gen () -> Result<[u8; 1312], MlDsaError>{
    let mut xi = [0u8; 32];
    let _ = getrandom::fill(&mut xi)
        .map_err(|_| MlDsaError::KegGenRandomSeedError);
    Ok(key_gen_internal(&xi))
}

fn to_polynomial_ring(s: &[i8; 256]) -> [i32; 256] {
    let t1 = s.map(|x| mod_q(x as i64));
    let t2 = s.map(|x| (x as i32) + (((x as i32) >> 31) & Q));
    assert_eq!(t1, t2);
    t2
}

// fn key_gen_internal (seed: &[u8; 32]) -> ([[[i32; 256]; L]; K], [[i32; 256]; K]) {
fn key_gen_internal (xi: &[u8; 32]) -> [u8; 1312] {
    let mut rho = [0u8; 32];
    let mut rho_prime = [0u8; 64];
    let mut k = [0u8; 32];

    {
        let mut d = [0u8; 34];
        d[..32].copy_from_slice(xi);
        d[32] = K as u8;
        d[33] = L as u8;

        let mut seed = [0u8; 128];
        sha3::Shake256::digest_xof(&d, &mut seed);

        rho.copy_from_slice(&seed[0..32]);
        rho_prime.copy_from_slice(&seed[32..96]);
        k.copy_from_slice(&seed[96..128]);
    }

    // Expand matrix.
    let a_hat = expand_a(&rho);
    // Sample short vectors s1 and s2.
    let (s1, s2): ([[i32; 256]; L], [[i32; 256]; K]) = expand_s(&rho_prime);
    let mut s1_hat = [[0i32; 256]; L];
    for (i, cv) in s1.iter().enumerate() {
        s1_hat[i] = ntt(&cv);
    }
    // multiply a_hat and s1_hat
    let mut prod_a_s1 =  [[0i32; 256]; K];
    for (i, a_row_poly) in a_hat.iter().enumerate() {
        for (k, s1_col_poly) in s1_hat.iter().enumerate() {
            prod_a_s1[i] = ntt_add(&prod_a_s1[i], &ntt_multiply(&a_row_poly[k], &s1_col_poly))
        }
    }
    let r = prod_a_s1.map(|v| ntt_inverse(&v));
    let t: [[i32; 256]; K] = std::array::from_fn(|i| vec_add(&r[i], &s2[i]));

    let mut t0 = [[0i32; 256]; K];
    let mut t1 = [[0i32; 256]; K];
    for (i, t_vec) in t.iter().enumerate() {
        for j in 0..256 {
            (t1[i][j], t0[i][j]) = power2round(t_vec[j])
        }
        for j in 0..256 {
            assert!(t1[i][j] < 1024_i32);
        }
    }

    let pk = pk_encode(&rho, &t1);
    // println!("public key: {}", hex::encode_upper(pk));
    pk
}

fn pk_encode(rho: &[u8; 32], t1: &[[i32; 256]; K]) -> [u8; 1312]{
    let mut pk = [0u8; 1312];
    pk[0..32].copy_from_slice(rho);
    for i in 0..K {
        let t_packed = simple_bit_pack(&t1[i], 1023); // 2^(bitlen(q-1)-d) - 1 = 2^10 - 1 = 1023
        pk[32+(i*320)..32+((i+1)*320)].copy_from_slice(&t_packed)
    }
    pk
}

fn simple_bit_pack(a: &[i32; 256], _b: u32) -> [u8; 320] {
    let mut r = [0u8; 320];
    for i in 0..N as usize/4 {
        r[5*i+0] = ((a[4*i+0]) >> 0) as u8;
        r[5*i+1] = (((a[4*i+0]) >> 8) | ((a[4*i+1]) << 2)) as u8;
        r[5*i+2] = (((a[4*i+1]) >> 6) | ((a[4*i+2]) << 4)) as u8;
        r[5*i+3] = (((a[4*i+2]) >> 4) | ((a[4*i+3]) << 6)) as u8;
        r[5*i+4] = ((a[4*i+3]) >> 2) as u8;
    }
    r
}

fn vec_add(a: &[i32; 256], b: &[i32; 256]) -> [i32; 256] {
    ntt_add(a, b)
}

fn expand_a(seed: &[u8; 32]) -> [[[i32; 256]; L]; K] {
    let mut a_hat: [[[i32; 256]; L]; K] = [[[0i32; 256]; L]; K];
    let mut rp = [0u8; 34];
    rp[0..32].copy_from_slice(seed);
    for r in 0..K {
        for s in 0..L {
            rp[32] = s as u8;
            rp[33] = r as u8;
            let z_q = reg_ntt_poly(rp);
            a_hat[r][s] = z_q;
        }
    }
    a_hat
}

fn expand_s(r: &[u8; 64]) -> ([[i32; 256]; L], [[i32; 256]; K]) {
    let mut s1 = [[0i32; 256]; L];
    let mut s2 = [[0i32; 256]; K];
    let mut rho = [0u8; 66];
    rho[0..64].copy_from_slice(r);
    rho[65] = 0; // rho[64..65] = IntegerToBytes(r, 2)
    for r in 0..L {
        rho[64] = r as u8;
        s1[r] = reg_bounded_poly(rho)
    }
    for r in 0..K {
        rho[64] = (r + L) as u8;
        s2[r] = reg_bounded_poly(rho);
    }
    (s1, s2)
}

// q = 2^23 - 2^13 + 1 = 8380417.
// returns an element in Z_q which fits in 3 bytes.
// this is a polynomial in the NTT form.
fn reg_ntt_poly(seed:[u8; 34]) -> [i32; 256] {
    let mut j = 0usize;
    let mut g = sha3::Shake128::default();
    g.update(&seed);
    let mut xof = g.finalize_xof();
    let mut poly = [0i32; 256];
    while j < 256 {
        let mut s = [0u8; 3];
        xof.read(&mut s);
        if let Ok(z_q) = coefficient_from_three_bytes(s[0], s[1], s[2]) {
            assert!(z_q < Q);
            poly[j] = z_q; // mod_q(z_q as i64);
            j += 1;
        }
    }
    poly
}

fn reg_bounded_poly(rho: [u8; 66]) -> [i32; 256] {
    let mut poly = [0i8; 256];
    let mut h = sha3::Shake256::default();
    h.update(&rho);
    let mut xof = h.finalize_xof();
    let mut j = 0usize;
    while j < 256 {
        let mut z = [0u8];
        xof.read(&mut z);
        let rz0 = coefficient_from_half_byte(z[0] & 0x0F);
        let rz1 = coefficient_from_half_byte((z[0] >> 4) & 0x0F);
        if let Ok(z0) = rz0 {
            poly[j] = z0;
            j += 1;
        }
        if let Ok(z1) = rz1 && j < 256 {
            poly[j] = z1;
            j += 1;
        }
    }
    to_polynomial_ring(&poly)
}

fn coefficient_from_half_byte(b: u8) -> Result<i8, MlDsaError> {
    assert_eq!(ETA, 2);
    const MOD5: [i8; 16] = [0,1,2,3,4,0,1,2,3,4,0,1,2,3,4,0];
    if ETA == 2 && b < 15 {
        Ok(2 - MOD5[(b & 0x0F) as usize])
    } else {
        Err(MlDsaError::BoundedPolySampleError)
    }
}

// generates an element of {0, 1, 2,..., q-1} + { NTTPolySampleError }
fn coefficient_from_three_bytes(b0: u8, b1: u8, b2: u8) -> Result<i32, MlDsaError> {
    let b2 = (b2 & 127) as i32; // set the top bit of b2 to 0, such that 0 <= b <= 127
    let z = (b2 << 16) + ((b1 as i32) << 8) + (b0 as i32);
    if z < Q {
        Ok(z)
    } else {
        Err(MlDsaError::NTTPolySampleError)
    }
}

fn power2round(r:i32) -> (i32, i32) {
    assert!(r < Q);
    let r1 = (r + (1 << (D-1)) - 1) >> D;
    let r0 = r - (r1 << D);
    assert!(r1 < 1024);
    (r1, r0)
}


// NIST ML-DSA-44 ACVP KATs.
// https://github.com/usnistgov/ACVP-Server/blob/master/gen-val/json-files/ML-DSA-keyGen-FIPS204/expectedResults.json
#[cfg(test)]
mod nist_avcp_ml_dsa_44_keygen_kats {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use crate::keypair::key_gen_internal;

    /// Reads a text file in the format:
    /// tcId = <hex>
    /// xi = <hex>
    /// pk = <hex>
    /// sk = <hex>
    /// (blank line between test cases)
    pub fn read_mldsa_44_nist_avcp_kats(path: &str) -> io::Result<Vec<(String, String, String, String)>> {
        let file = File::open(path).expect("cannot open file");
        let reader = BufReader::new(file);

        let mut results = Vec::new();
        let mut tid = String::new();
        let mut xi = String::new();
        let mut pk = String::new();
        let mut sk = String::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                // End of one test case
                if !tid.is_empty() {
                    results.push((tid.clone(), xi.clone(), pk.clone(), sk.clone()));
                    tid.clear();
                    xi.clear();
                    pk.clear();
                    sk.clear();
                }
                continue;
            }

            if let Some(val) = line.strip_prefix("tid = ") {
                tid = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("xi = ") {
                xi = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("pk = ") {
                pk = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("sk = ") {
                sk = val.trim().to_string();
            }
        }
        if !tid.is_empty() {
            results.push((tid, xi, pk, sk));
        }
        Ok(results)
    }

    #[test]
    fn key_gen_avcp_nist_ml_dsa_44_tests() {
        let kats = read_mldsa_44_nist_avcp_kats("./kats/nist-acvp-keygen-kats.txt").unwrap();
        for (_id, kat_xi, kat_pk, kat_sk) in kats.iter() {
            let xi: [u8; 32] = hex::decode(&kat_xi).unwrap().try_into().unwrap();
            let kat_pk: [u8; 1312] = hex::decode(&kat_pk).unwrap().try_into().unwrap();
            let _kat_sk: [u8; 2560] = hex::decode(&kat_sk).unwrap().try_into().unwrap();
            let pk = key_gen_internal(&xi);
            assert_eq!(pk, kat_pk);
        }
    }
}

#[cfg(test)]
mod misc_ml_dsa_44_keygen_kats {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use crate::keypair::key_gen_internal;

    /// Reads a NIST ML-DSA KAT text file (no blank lines).
    /// Each test starts with `count = N`, followed by `seed`, `pk`, and `sk`.
    pub fn read_mldsa_hedged_kats(path: &str) -> io::Result<Vec<(String, String, String, String)>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut results = Vec::new();
        let mut tid = String::new();
        let mut xi = String::new();
        let mut pk = String::new();
        let mut sk = String::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if let Some(val) = line.strip_prefix("count = ") {
                // If previous test exists, push it before starting a new one
                if !tid.is_empty() {
                    results.push((tid.clone(), xi.clone(), pk.clone(), sk.clone()));
                    xi.clear();
                    pk.clear();
                    sk.clear();
                }
                tid = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("xi = ") {
                xi = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("pk = ") {
                pk = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("sk = ") {
                sk = val.trim().to_string();
            }
        }

        // Push the final entry if the file didn’t end with another "count ="
        if !tid.is_empty() {
            results.push((tid, xi, pk, sk));
        }

        Ok(results)
    }

    #[test]
    fn key_gen_misc_ml_dsa_44_tests() {
        let kats = read_mldsa_hedged_kats("./kats/misc-keygen-kats.txt").unwrap();
        for (_id, kat_xi, kat_pk, kat_sk) in kats.iter() {
            let xi: [u8; 32] = hex::decode(&kat_xi).unwrap().try_into().unwrap();
            let kat_pk: [u8; 1312] = hex::decode(&kat_pk).unwrap().try_into().unwrap();
            let _kat_sk: [u8; 2560] = hex::decode(&kat_sk).unwrap().try_into().unwrap();
            let pk = key_gen_internal(&xi);
            assert_eq!(pk, kat_pk);
        }
    }
}


#[cfg(test)]
mod keypair_tests {
    use crate::keypair::to_polynomial_ring;
    use crate::params::Q;

    #[test]
    fn test_it_to_ring() {
        let mut a = [0i8; 256];
        let mut b = [0i32; 256];
        for i in 1..128 {
            a[i] = -(i as i8);
            b[i] = Q - (i as i32);
        }

        let r = to_polynomial_ring(&a);
        assert_eq!(r, b);
    }
}