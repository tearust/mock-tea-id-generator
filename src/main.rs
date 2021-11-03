use rand::rngs::OsRng;
use rand::RngCore;

const TEA_ID_LEN: usize = 32;

fn main() {
    let mut seed = [0u8; TEA_ID_LEN];
    OsRng.fill_bytes(&mut seed);

    println!("Seed hex encoded (input in wallet when planting seed): ");
    println!("0x{}", hex::encode(seed.to_vec()));

    println!();

    println!("Seed base64 encoded (input in your server when setting machine id): ");
    println!("{}", base64::encode(seed.to_vec()));
}
