use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha512};



fn main () {
    let now = SystemTime::now();
    let seed = now.duration_since(UNIX_EPOCH.into()).unwrap().as_millis() as u64;
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(seed);
    let mut keys = Vec::new();
    let mut hashes: Vec<String> = Vec::new();



    for _ in 0..20 {
        let key: [u8; 32] = rng.gen();
        let hex_string: String = format!("{:02X?}", key).replace(", ", "").replace("[", "").replace("]", "");
        let input = Sha512::digest(hex_string.as_bytes());
        let hash = format!("{:x}", input);
        hashes.push(hash);
        keys.push(hex_string);
    }

    // save the keys and hashes to a file
    let mut seed = format!("{}\n", seed);
    let keys = keys.join("\n");
    seed.push_str(&keys);
    let hashes = hashes.join("\n");
    std::fs::write("keys.txt", seed).unwrap();
    std::fs::write("hashes.txt", hashes).unwrap();
}