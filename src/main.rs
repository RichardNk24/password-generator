use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <length>", args[0]);
        std::process::exit(1);
    }

    let length: usize = args[1].parse().expect("Length must be a number");

    let password = generate_password(length);
    println!("Generated password: {}", password);
}

fn generate_password(length: usize) -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789)(*&^%$#@!~";

    let mut rng = rand::thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    password
}
