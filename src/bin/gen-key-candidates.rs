use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, SeedableRng};
use rayon::iter::{ParallelBridge, ParallelIterator};


const SECOND_FRACTION: u128 = 1_000;

fn main() {
    // Get number of days from command line
    let days = std::env::args().nth(1).expect("Usage: gen_key_candidates <hours> <?start_unix_time>");
    let days = days.parse::<u64>().unwrap();
    // Get start time in unix seconds from command line
    let start_time = std::env::args().nth(2).unwrap_or(format!("{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));
    let start_time = start_time.parse::<u64>().unwrap();
    // Get the end time in unix seconds
    let end_time = start_time - (days * 60 * 60);

    let end_seed = start_time as u128 * SECOND_FRACTION;
    let start_seed = end_time as u128 * SECOND_FRACTION;
    println!("Generating keys from {} to {}", start_seed, end_seed);
    let seeds = (start_seed..end_seed).rev().map(|x| x as u64);

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("key_candidates.txt")
        .unwrap();

    // clear the file
    let _ = file.set_len(0);

    let file = BufWriter::new(file);
    let file = Mutex::new(file);

    seeds.par_bridge().for_each(|seed| {
        let mut rng = rand_chacha::ChaChaRng::seed_from_u64(seed);
        let key: [u8; 32] = rng.gen();
        let mut hex_string: String = format!("{:02X?}", key).replace(", ", "").replace("[", "").replace("]", "");
        hex_string.push('\n');
        file.lock().unwrap().write_all(hex_string.as_bytes()).unwrap();
    });

    println!("Done calculating keys");

    file.lock().unwrap().flush().unwrap();
}