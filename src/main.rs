mod formats;

use std::io::stdout;
use clap::Parser;
use crate::formats::{csharp_text, java_text};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    text: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
    //
    #[arg(short, long = "java", default_value_t = false)]
    is_java: bool,
    #[arg(short, long = "csharp", default_value_t = false)]
    is_csharp: bool,
}

fn main() {
    let args = Args::parse();
    let words = args.text;
    let first_word = &words[..words.find(' ').unwrap_or(words.len())];
    let other_text = &words[words.find(' ').unwrap_or(words.len())+1..];
    if args.is_java {
        println!("{}", java_text(first_word, other_text));
    } else if args.is_csharp {
        println!("{}", csharp_text(first_word, other_text));
    } else {
        println!("{}", java_text(first_word, other_text));
    }
}