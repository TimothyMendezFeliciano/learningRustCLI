#![allow(unused)]
use clap::{arg, Parser};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("Could Not Read File");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    // let pattern = std::env::args().nth(1).expect("No Pattern Given");
    // let path = std::env::args().nth(2).expect("No Path Given");
    // let args = Cli {
    //     path: std::path::PathBuff::from(path),
    //     pattern,
    // };
    // println!("Hello, world!");
}
