use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("key_candidates.txt")?;
    let reader = BufReader::new(file);
    let mut keys = String::new();
    let _ = File::open("keys.txt")?.read_to_string(&mut keys)?;
    let mut hashes = String::new();
    let _ = File::open("hashes.txt")?.read_to_string(&mut hashes)?;
    let keys = keys.lines().map(|s| s.to_string()).skip(1).collect::<Vec<String>>();

    for line in reader.lines() {
        let line = line?;
        if keys.contains(&line) {
            println!("Key found: {}", line);
        }
    }
    Ok(())
}
