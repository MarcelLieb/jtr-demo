use md5::{Digest, Md5};

static SECRET: &str = "db49f02a426bf605bd8a708500d6dcfa";

fn main() {
    let input = std::env::args().nth(1).expect("Usage: demo <key>");

    let mut hasher = Md5::new();


    hasher.update(input);

    let input = hasher.finalize();

    if format!("{:x}", input) != SECRET {
        println!("Invalid key");
        return;
    }

    println!("Congratulations! The flag is: {}", std::env::var("FLAG").unwrap_or("FLAG not set".to_string()));
}
