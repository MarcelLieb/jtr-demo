use std::fs::{self};

use sha2::{Sha512, Digest};

// digest of the hashes.txt file
static SECRET: &str = "6be14066aec127466cbac668dd6d4143168da39e6742b5f3b85c8f468e1185e5f20331f107ae339486f917eabfa06801430a082fb39563d2ffc430266ddb8709";

fn main() -> std::io::Result<()> {
    let input = std::env::args().nth(1).expect("Usage: demo <key>");
    let file = fs::read_to_string("hashes.txt")?;
    let hashes = file.lines().map(|s| s.to_string()).collect::<Vec<String>>();


    let verification = Sha512::digest(file.as_bytes());
    let verification = format!("{:x}", verification);

    if !SECRET.is_empty() && verification != SECRET {
        println!("Invalid key list");
        return Ok(());
    }
    
    // Hash input
    let input = Sha512::digest(input.as_bytes());
    let input = format!("{:x}", input);

    if !hashes.contains(&input) {
        println!("Invalid key");
        return Ok(());
    }

    println!("Congratulations! The flag is: {}", std::env::var("FLAG").unwrap_or("FLAG not set".to_string()));
    Ok(())
}
