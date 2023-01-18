use clap::Parser;
use rand_core::OsRng;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use x25519_dalek::{PublicKey, StaticSecret};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// path to private key file
    #[arg(short, long)]
    secret: String,

    /// path to public key file
    #[arg(short, long)]
    public: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("private key: {}, public key: {}", cli.secret, cli.public);

    let secret = StaticSecret::new(OsRng);
    let public = PublicKey::from(&secret);

    let mut file = File::create(cli.secret)?;
    file.write_all(&secret.to_bytes())?;

    let mut file = File::create(cli.public)?;
    file.write_all(&public.to_bytes())?;

    println!("Generation successful!");

    Ok(())
}

