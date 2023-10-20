use clap::Parser;
use rand;
use secp256k1::SecretKey;
use std::error::Error;
use std::{fs::File, io::Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// path to private key file
    #[arg(short, long)]
    secretpath: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("secret key path: {}", cli.secretpath);
    let mut rng = rand::thread_rng();

    let seckey = SecretKey::new(&mut rng);
    let mut file = File::create(cli.secretpath)?;
    file.write_all(&seckey.secret_bytes())?;
    Ok(())
}
