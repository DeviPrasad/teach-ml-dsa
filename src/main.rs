mod keypair;
mod err;
mod params;
mod ntt;

fn main() {
    let _ = keypair::key_gen().unwrap();
}
