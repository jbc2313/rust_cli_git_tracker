#![allow(unused)]
use clap::Parser;
use std::io::{Write, stdout, stdin, self};
use std::time::Duration;
use std::thread;
use indicatif::ProgressBar;

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
    
    let pb = ProgressBar::new(1024);
    for _ in 0..1024 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(4));
    };
    pb.finish_with_message("done!!");
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "foo: {}", 23);

}
