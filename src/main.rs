use clap::Parser;
use libsodium_sys::{crypto_sign_keypair, sodium_init, sodium_memzero};
use rand;
use secp256k1::SecretKey;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// path to private key file
    #[arg(short, long)]
    secret: String,

    /// path to public key file
    #[arg(short, long)]
    public: String,

    /// path to secp private key file
    #[arg(short = 'k', long)]
    secpsecret: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!(
        "private key: {}, public key: {}, secp private key: {}",
        cli.secret, cli.public, cli.secpsecret
    );

    let mut secret = [0u8; 64];
    let mut public = [0u8; 32];

    if unsafe { sodium_init() } < 0 {
        return Err("failed to init libsodium".into());
    }

    if unsafe { crypto_sign_keypair(public.as_mut_ptr(), secret.as_mut_ptr()) } != 0 {
        return Err("failed to generate keypair".into());
    }

    let mut file = File::create(cli.secret)?;
    file.write_all(&secret)?;

    let mut file = File::create(cli.public)?;
    file.write_all(&public)?;

    unsafe {
        sodium_memzero(secret.as_mut_ptr().cast(), 64);
    }

    // generating secp private key
    let mut rng = rand::thread_rng();
    let seckey = SecretKey::new(&mut rng);
    let mut file = File::create(cli.secpsecret)?;
    file.write_all(&seckey.secret_bytes())?;

    println!("Generation successful!");

    Ok(())
}
