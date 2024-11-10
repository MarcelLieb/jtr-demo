use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("key_candidates.txt")?;
    let reader = BufReader::new(file);
    let keys = fs::read_to_string("keys.txt")?;
    let _hashes = fs::read_to_string("hashes.txt")?;
    let keys = keys.lines().map(|s| s.to_string()).skip(1).collect::<Vec<String>>();

    for line in reader.lines() {
        let line = line?;
        if keys.contains(&line) {
            println!("Key found: {}", line);
        }
    }
    Ok(())
}
