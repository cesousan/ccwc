use anyhow::{Context, Result};
use clap::Parser;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// Number of bytes in file
    #[clap(short = 'c', long)]
    bytes: bool,

    /// Number of lines in file
    #[clap(short = 'l', long)]
    lines: bool,

    /// Number of words in file
    #[clap(short = 'w', long)]
    words: bool,

    /// Number of chars in file
    #[clap(short = 'm', long)]
    chars: bool,

    /// The path to the file to read
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut result = String::new();
    // check if no args are passed except the path
    if !args.bytes && !args.lines && !args.words && !args.chars {
        let content = get_content(&args.path)?;
        result += &num_lines(&content)?.to_owned();
        result += " ";
        result += &num_words(&content)?.to_owned();
        result += " ";
        result += &num_bytes(&content)?.to_owned();
        result += " ";
    } else {
        let content = get_content(&args.path)?;
        if args.bytes {
            result += &num_bytes(&content)?.to_owned();
            result += " ";
        }
        if args.lines {
            result += &num_lines(&content)?.to_owned();
            result += " ";
        }
        if args.words {
            result += &num_words(&content)?.to_owned();
            result += " ";
        }
        if args.chars {
            result += &num_chars(&content)?.to_owned();
            result += " ";
        }
    }
    let file_name = match &args.path {
        Some(path) => path.file_name().unwrap().to_str().unwrap(),
        None => "",
    };
    println!("{}{}", result, file_name);
    Ok(())
}

fn get_content(path: &Option<PathBuf>) -> Result<String> {
    match path {
        Some(path) => std::fs::read_to_string(path)
            .with_context(|| format!("could not read file `{}`", path.display())),
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}
fn num_chars(content: &String) -> Result<String> {
    let chars = content.chars().count().to_string();
    Ok(chars)
}

fn num_words(content: &String) -> Result<String> {
    let words = content.split_whitespace().count().to_string();
    Ok(words)
}
fn num_lines(content: &String) -> Result<String> {
    let lines = content.lines().count().to_string();
    Ok(lines)
}
fn num_bytes(content: &String) -> Result<String> {
    let bytes = content.len().to_string();
    Ok(bytes)
}
