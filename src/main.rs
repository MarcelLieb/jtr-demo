use std::fs::{self};

use sha2::{Sha512, Digest};

// digest of the hashes.txt file
static SECRET: &str = "3296d928cca0935e65c24c3cfa70a43104bc4b13030563b8b93cce6c558a39aad92dfd863675e82cff3fccc9883ffdffdb748ce98f25b65d249edbc3db2c9651";

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
