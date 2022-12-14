use anyhow::{Context, Result};
use clap::Parser;
use grrs::find_matches;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[test]
fn check_answer_validity() {
    // assert_eq!(answer(), 42)
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;
    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
//
// fn main() -> Result<()> {
//     let path = "test.txt";
//     let content = std::fs::read_to_string(path)
//         .with_context(|| format!("Could not read file `{}`", path))?;
//     println!("File Content {}", content);
//     Ok(())
// }


// #![allow(unused)]
//
// use clap::{arg, Parser};
//
// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     path: std::path::PathBuf,
// }
//
// #[derive(Debug)]
// struct CustomError(String);
//
// fn main() -> Result<(), CustomError> {
//     let path = "test.txt";
//     let content = std::fs::read_to_string(path)
//         .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
//     println!("file content: {}", content);
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let content = std::fs::read_to_string("test.txt")?;
//     println!("file content: {}", content);
//     Ok(())
// let args = Cli::parse();
// let result = std::fs::read_to_string("test.txt");
// let content = match result {
//     OK(content) => { content }
//     Err(error) => { return Err(error.into()); }
// };
// println!("file content: {}", content);
// Ok(())
// for line in content.lines() {
//     if line.contains(&args.pattern) {
//         println!("{}", line);
//     }
// }
// let pattern = std::env::args().nth(1).expect("No Pattern Given");
// let path = std::env::args().nth(2).expect("No Path Given");
// let args = Cli {
//     path: std::path::PathBuff::from(path),
//     pattern,
// };
// println!("Hello, world!");
// }
