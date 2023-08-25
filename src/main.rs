use clap::Parser;
use std::sync::mpsc::channel;
use std::time::Instant;
use std::{println, thread};

use sha256::digest;

const MAX_NONCE: usize = std::u32::MAX as usize;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short)]
    pub f: usize,
    #[arg(short)]
    pub n: usize,
}

fn sha256(input: usize) -> String {
    digest(&input.to_ne_bytes())
}

fn count_trailing_zeros(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars().rev() {
        if c == '0' {
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn main() {
    let config = Config::parse();
    let num_threads = num_cpus::get();
    let start_time = Instant::now();

    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut nonce = i;
            let nonce_step = num_threads;
            while nonce < MAX_NONCE {
                let hash = sha256(nonce);

                if count_trailing_zeros(&hash) == config.n {
                    tx.send((nonce, hash)).unwrap();
                }

                nonce += nonce_step;
            }
        });
    }
    for _ in 0..config.f {
        let j = rx.recv().unwrap();
        println!("{}, {}", j.0, j.1)
    }
    let elapsed_time = start_time.elapsed();
    println!(
        "Time taken: {}.{} seconds",
        elapsed_time.as_secs(),
        elapsed_time.as_millis()
    );
}
