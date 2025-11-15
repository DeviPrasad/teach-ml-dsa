use crate::err::MlDsaError;
use crate::params::{N, D, ETA, K, L, LEN_ETA_PACK_POLY, LEN_PRIVATE_KEY, LEN_T0_PACK_POLY};

pub fn sk_decode(sk: &[u8; LEN_PRIVATE_KEY]) -> Result<([u8; 32], [u8; 32], [u8; 64], [[i32; 256]; L], [[i32; 256]; K], [[i32; 256]; K]), MlDsaError> {
    assert_eq!(sk.len(), 128 + L * LEN_ETA_PACK_POLY + K * LEN_ETA_PACK_POLY + K * LEN_T0_PACK_POLY);
    let rho = sk[0..32].try_into()?;
    let key = sk[32..64].try_into()?;
    let tr = sk[64..128].try_into()?;
    let y: &[u8; L * LEN_ETA_PACK_POLY] = &sk[128..128 + L*LEN_ETA_PACK_POLY].try_into()?;
    let z: &[u8; K * LEN_ETA_PACK_POLY] = &sk[128 + L*LEN_ETA_PACK_POLY..128 + L*LEN_ETA_PACK_POLY + K*LEN_ETA_PACK_POLY].try_into()?;
    let w: &[u8; K * LEN_T0_PACK_POLY] = &sk[128 + L*LEN_ETA_PACK_POLY + K*LEN_ETA_PACK_POLY ..].try_into()?;

    let mut s1 = [[0i32; 256]; L];
    for i in 0..L {
        let t = &y[i * LEN_ETA_PACK_POLY..(i + 1) * LEN_ETA_PACK_POLY].try_into()?;
        bit_unpack_eta(&t, &mut s1[i])?;
    }

    let mut s2: [[i32; 256]; K] = [[0; 256]; K];
    for i in 0..K {
        let t = &z[i * LEN_ETA_PACK_POLY..(i + 1) * LEN_ETA_PACK_POLY].try_into()?;
        bit_unpack_eta(&t, &mut s2[i])?;
    }

    let mut t0: [[i32; 256]; K] = [[0; 256]; K];
    for i in 0..K {
        let t = &w[i * LEN_T0_PACK_POLY..(i + 1) * LEN_T0_PACK_POLY].try_into()?;
        bit_unpack_t0(&t, &mut t0[i])?;
    }
    Ok((rho, key, tr, s1, s2, t0))
}

fn inclusive(l: i32, h: i32, v: i32) -> bool {
    (l..h+1).contains(&v)
}

// w is bit-packed polynomial
fn bit_unpack_eta(w: &[u8; LEN_ETA_PACK_POLY], s: &mut[i32; 256]) -> Result<(), MlDsaError> {
    const _ETA_: i32 = ETA as i32;
    let mut ok = true;

    #[cfg(any(feature="ML_DSA_44", feature="ML_DSA_87"))]
    for i in 0..N as usize/8 {
        s[8*i+0] = ((w[3*i+0] >> 0) & 7) as i32;
        s[8*i+1] = ((w[3*i+0] >> 3) & 7) as i32;
        s[8*i+2] = (((w[3*i+0] >> 6) | (w[3*i+1] << 2)) & 7) as i32;
        s[8*i+3] = ((w[3*i+1] >> 1) & 7) as i32;
        s[8*i+4] = ((w[3*i+1] >> 4) & 7) as i32;
        s[8*i+5] = (((w[3*i+1] >> 7) | (w[3*i+2] << 1)) & 7) as i32;
        s[8*i+6] = ((w[3*i+2] >> 2) & 7) as i32;
        s[8*i+7] = ((w[3*i+2] >> 5) & 7) as i32;

        s[8*i+0] = _ETA_ - s[8*i+0];
        s[8*i+1] = _ETA_ - s[8*i+1];
        s[8*i+2] = _ETA_ - s[8*i+2];
        s[8*i+3] = _ETA_ - s[8*i+3];
        s[8*i+4] = _ETA_ - s[8*i+4];
        s[8*i+5] = _ETA_ - s[8*i+5];
        s[8*i+6] = _ETA_ - s[8*i+6];
        s[8*i+7] = _ETA_ - s[8*i+7];
        ok &= inclusive(-_ETA_, _ETA_, s[2*i+1]) & inclusive(-_ETA_, _ETA_, s[2*i+1]) &
            inclusive(-_ETA_, _ETA_, s[2*i+2]) & inclusive(-_ETA_, _ETA_, s[2*i+1]) &
            inclusive(-_ETA_, _ETA_, s[2*i+5]) & inclusive(-_ETA_, _ETA_, s[2*i+1]) &
            inclusive(-_ETA_, _ETA_, s[2*i+7]) & inclusive(-_ETA_, _ETA_, s[2*i+1]);
    }

    #[cfg(feature="ML_DSA_65")]
    for i in 0..N as usize/2 {
        s[2*i+0] = (w[i] & 0x0F) as i32;
        s[2*i+1] = (w[i] >> 4) as i32;
        s[2*i+0] = _ETA_ - s[2 * i + 0];
        s[2*i+1] = _ETA_ - s[2 * i + 1];
        ok &= inclusive(-_ETA_, _ETA_, s[2 * i + 0]) & inclusive(-_ETA_, _ETA_, s[2 * i + 1]);
    }

    if ok {
        Ok(())
    } else {
        Err(MlDsaError::MalformedShortVector)
    }
}

/**
One iteration handles bits in this fashion - w0 is the first byte to consider in the loop.

|         w3         |              w2             |          w1         |        w0           |
|------+------+------|+------+------+------+-------|+------+------+------|+------+------+------|
|             |  8   |   5   |  13  |  13  |   1   |   12  |  13  |   7  |   6   |  13  |  13  |
|------+------+------|+------+------+------+-------|+------+------+------|+------+------+------|
|                    |                             |                     |                     |
 **/
// Unpack polynomial t0 with coefficients in ]-2^{D-1}, 2^{D-1}]
fn bit_unpack_t0(w: &[u8; LEN_T0_PACK_POLY], t: &mut [i32; 256]) -> Result<(), MlDsaError> {
    const _2_POW_D_MINUS_1_: i32 = 1 << (D-1);
    let mut ok = true;
    for i in 0..N as usize/8 {
        t[8*i+0] = w[13*i+0] as i32;
        t[8*i+0] |= (w[13*i+1] as i32) << 8;
        t[8*i+0] &= 0x1FFF;

        t[8*i+1] = (w[13*i+1] as i32) >> 5;
        t[8*i+1] |= (w[13*i+2] as i32) << 3;
        t[8*i+1] |= (w[13*i+3] as i32) << 11;
        t[8*i+1] &= 0x1FFF;

        // w0 = | 6 | 13 | 13 |
        let w0 = i32::from_le_bytes(w[13*i..13*i+4].try_into()?);
        assert_eq!(w0 & 0x1FFF, t[8*i+0]);
        assert_eq!((w0 >> 13) & 0x1FFF, t[8*i+1]);
        let w0 = (w0 >> 26) & 0x3F;

        t[8*i+2] = (w[13*i+3] as i32) >> 2;
        t[8*i+2] |= (w[13*i+4] as i32) << 6;
        t[8*i+2] &= 0x1FFF;

        t[8*i+3] = (w[13*i+4] as i32) >> 7;
        t[8*i+3] |= (w[13*i+5] as i32) << 1;
        t[8*i+3] |= (w[13*i+6] as i32) << 9;
        t[8*i+3] &= 0x1FFF;

        //                w0 = |  6 | 13 | 13 |
        // w1 = | 12 | 13 |  7 |
        let w1 = i32::from_le_bytes(w[13*i+4..13*i+8].try_into()?);
        assert_eq!(((w1 & 0x7F) << 6) | w0, t[8*i+2]);
        assert_eq!((w1 >> 7) & 0x1FFF, t[8*i+3]);
        let w1 = (w1 >> 20) & 0xFFF;

        t[8*i+4] = (w[13*i+6] as i32) >> 4;
        t[8*i+4] |= (w[13*i+7] as i32) << 4;
        t[8*i+4] |= (w[13*i+8] as i32) << 12;
        t[8*i+4] &= 0x1FFF;

        t[8*i+5] = (w[13*i+8] as i32) >> 1;
        t[8*i+5] |= (w[13*i+9] as i32) << 7;
        t[8*i+5] &= 0x1FFF;

        //                                    w0 = |  6 | 13 | 13 |
        //                     w1 = | 12 | 13 |  7 |
        // w2 = |  5 | 13 | 13 |  1 |
        let w2 = i32::from_le_bytes(w[13*i+8..13*i+12].try_into()?);
        assert_eq!(((w2 & 0x1) << 12) | w1, t[8*i+4]);
        assert_eq!((w2 >> 1) & 0x1FFF, t[8*i+5]);
        let w2 = w2 >> 14; // w2 = | 0 | 5 |  13 |

        t[8*i+6] = (w[13*i+9] as i32) >> 6;
        t[8*i+6] |= (w[13*i+10] as i32) << 2;
        t[8*i+6] |= (w[13*i+11] as i32) << 10;
        t[8*i+6] &= 0x1FFF;

        t[8*i+7] = (w[13*i+11] as i32) >> 3;
        t[8*i+7] |= (w[13*i+12] as i32) << 5;
        t[8*i+7] &= 0x1FFF;

        //                w2 = | 5 | 13 |
        // w3 = | 11 | 13 |  8 |
        // we only use the least-significant byte of w3, and finish one iteration.
        assert_eq!(w2 & 0x1FFF, t[8 * i + 6]);
        let w2 = (w2 >> 13) & 0x1F; // w2 = | 0 | 5 | 13 | >> 13 = | 0 | 5 |
        assert_eq!(((w[13 * i + 12] as i32) << 5) | (w2 & 0x1F), t[8*i+7]);

        t[8*i+0] = _2_POW_D_MINUS_1_ - t[8*i+0];
        t[8*i+1] = _2_POW_D_MINUS_1_ - t[8*i+1];
        t[8*i+2] = _2_POW_D_MINUS_1_ - t[8*i+2];
        t[8*i+3] = _2_POW_D_MINUS_1_ - t[8*i+3];
        t[8*i+4] = _2_POW_D_MINUS_1_ - t[8*i+4];
        t[8*i+5] = _2_POW_D_MINUS_1_ - t[8*i+5];
        t[8*i+6] = _2_POW_D_MINUS_1_ - t[8*i+6];
        t[8*i+7] = _2_POW_D_MINUS_1_ - t[8*i+7];
    }
    for i in 0..N as usize {
        ok &= inclusive(-_2_POW_D_MINUS_1_+1, _2_POW_D_MINUS_1_, t[i]);
    }
    if ok {
        Ok(())
    } else {
        Err(MlDsaError::MalformedShortVector)
    }
}