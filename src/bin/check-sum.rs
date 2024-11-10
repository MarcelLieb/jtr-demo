use sha2::Digest;



fn main() -> std::io::Result<()> {
    let file_path = std::env::args().nth(1).expect("Specify a file path");
    let file = std::fs::read_to_string(file_path)?;
    let hash = sha2::Sha512::digest(file.as_bytes());
    println!("{:x}", hash);
    Ok(())
}