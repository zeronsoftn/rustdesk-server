use sodiumoxide::crypto::sign;
use std::{
    env,
    process,
};

fn print_help() {
    println!(
        "Usage:
    rustdesk-util [command]\n
Available Commands:
    genkeypair                                   Generate a new keypair"
    );
    process::exit(0x0001);
}

fn gen_keypair() {
    let (pk, sk) = sign::gen_keypair();
    let public_key = base64::encode(pk);
    let secret_key = base64::encode(sk);
    println!("Public Key:  {public_key}");
    println!("Secret Key:  {secret_key}");
}


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        print_help();
    }

    let command = args[1].to_lowercase();
    match command.as_str() {
        "genkeypair" => gen_keypair(),
        _ => print_help(),
    }
}
