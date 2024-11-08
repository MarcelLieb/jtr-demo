use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::BTreeSet;

use rand::{Rng, SeedableRng};
use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelIterator};




#[tokio::main]
async fn main() {
    // Get start time in unix seconds from command line
    let start_time = std::env::args().nth(2).unwrap_or(format!("{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));
    let start_time = start_time.parse::<u64>().unwrap();
    // Get number of days from command line
    let days = std::env::args().nth(1).expect("Usage: gen_key_candidates <days> <start_time>");
    let days = days.parse::<u64>().unwrap();
    // Get the end time in unix seconds
    let end_time = start_time - (days * 86400);

    let end_seed = start_time as u128 * 1000;
    let start_seed = end_time as u128 * 1000;
    let seeds = (start_seed..end_seed).map(|x| x as u64).collect::<Vec<u64>>();
    let seeds = BTreeSet::<u64>::from_par_iter(seeds.into_par_iter());
    println!("Generating {} seeds", seeds.len());

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("key_candidates.txt")
        .await
        .unwrap();
    file.set_len(0).await.unwrap();
    let file = Arc::new(Mutex::new(file));

    let (tx, rx) = kanal::unbounded::<String>();

    let file_writer = file.clone();

    for _ in 0..4 {
        let file = file_writer.clone();
        let rx = rx.clone();
        tokio::spawn(async move {
            // multi thread buffered write
            let mut buffer = Vec::new();
            while let Ok(hex_string) = rx.recv() {
                buffer.push(hex_string);
                if buffer.len() == 1 << 10 {
                    let mut file = file.lock().await;
                    let mut joined = buffer.join("\n");
                    joined.push('\n');
                    file.write_all(joined.as_bytes()).await.unwrap();
                    buffer.clear();
                }
            }
        });
    }


    seeds.into_par_iter().for_each(|seed| {
        let mut rng = rand_chacha::ChaChaRng::seed_from_u64(seed);
        let key: [u8; 32] = rng.gen();
        let hex_string: String = format!("{:02X?}", key).replace(", ", "").replace("[", "").replace("]", "");
        let tx = tx.clone();
        tx.send(hex_string).unwrap();
    });
}