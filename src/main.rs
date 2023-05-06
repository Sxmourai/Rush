mod cat;
mod ls;
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
        _ => panic!("Program required !"),
    }
}