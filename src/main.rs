#![allow(unused)]
use clap::Parser;
use std::io::{Write, stdout, stdin};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() {
    println!("Hello, world!");
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file..");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }



}
