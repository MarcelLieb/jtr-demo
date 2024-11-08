use std::{fs::File, io::Read};

use sha2::{Sha512, Digest};

// digest of the hashes.txt file
static SECRET: &str = "";

fn main() -> std::io::Result<()> {
    let input = std::env::args().nth(1).expect("Usage: demo <key>");
    let mut file = String::new();
    let _ = File::open("hashes.txt")?.read_to_string(&mut file)?;
    let hashes = file.lines().map(|s| s.to_string()).collect::<Vec<String>>();


    let mut hasher = Sha512::new();
    hasher.update(file);
    let verification = hasher.finalize();
    let verification = format!("{:x}", verification);

    if !SECRET.is_empty() && verification != SECRET {
        println!("Invalid key list");
        return Ok(());
    }
    
    // Hash input
    let mut hasher = Sha512::new();
    hasher.update(input);
    let input = hasher.finalize();

    if !hashes.contains(&format!("{:x}", input)) {
        println!("{:x}", input);
        println!("Invalid key");
        return Ok(());
    }

    println!("Congratulations! The flag is: {}", std::env::var("FLAG").unwrap_or("FLAG not set".to_string()));
    Ok(())
}
