mod cat;
mod ls;
mod clipboard;
mod utils;
use clap::Parser;


#[derive(Parser, Debug)]
struct Args {
    execute: String,
    rest: Vec<String>,
}

fn main() {
    let args = Args::parse();
    match args.execute.as_str() {
        "cat" => cat::run(args.rest),
        "ls" => ls::run(args.rest),
        "clip" => clipboard::clip(args.rest),
        "paste" => clipboard::paste(args.rest),
        "history" => clipboard::history(),
        _ => panic!("Program required !"),
    }
}