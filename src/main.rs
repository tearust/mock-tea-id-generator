use rand::rngs::OsRng;
use rand::RngCore;

const TEA_ID_LEN: usize = 32;

fn main() {
    let mut seed = [0u8; TEA_ID_LEN];
    OsRng.fill_bytes(&mut seed);

    print!("{}", base64::encode(seed.to_vec()));
}
