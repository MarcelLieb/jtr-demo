use std::fs::{self};

use sha2::{Sha512, Digest};

// digest of the hashes.txt file
static SECRET: &str = "17ab6bfc53888ca47f7550a37aa9133ab30642c4f05741928a3539156b29ee311e97cf0f57d6a34489dc0bd23c8f9bcb9fa9d727c4ffdc933422ac33cadf5ac1";

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
