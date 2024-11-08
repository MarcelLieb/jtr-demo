use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use md5::{Digest, Md5};



fn main () {
    let now = SystemTime::now();
    let seed = now.duration_since(UNIX_EPOCH.into()).unwrap().as_millis();
    let seed_lower = seed as u64;
    let mut rng = ChaChaRng::seed_from_u64(seed_lower);
    let mut keys = Vec::new();
    let mut hashes: Vec<String> = Vec::new();



    for _ in 0..20 {
        let key: [u8; 32] = rng.gen();
        let hex_string: String = format!("{:02X?}", key).replace(", ", "").replace("[", "").replace("]", "");
        let mut hasher = Md5::new();
        hasher.update(hex_string.clone());
        let input = hasher.finalize();
        let hash = format!("{:x}", input);
        hashes.push(hash);
        keys.push(hex_string);
    }

    // save the keys and hashes to a file
    let keys = keys.join("\n");
    let hashes = hashes.join("\n");
    std::fs::write("keys.txt", keys).unwrap();
    std::fs::write("hashes.txt", hashes).unwrap();
}