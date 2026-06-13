use clap::{Parser};

use rand::{self, seq::IndexedRandom};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    digit: u32
}

pub fn cli() {
    let cli = Cli::parse();
    let pool: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+()[]/?!'@#$&*".chars().collect();

    let mut rng = rand::rng();
    let password: String = (0..cli.digit).map(|_| pool.choose(&mut rng).unwrap()).collect();

    println!("{}", password);
}

