use std::{env, io};
use std::env::current_dir;
use std::path::PathBuf;
use std::fs::{copy, File, read_to_string};
use std::io::{BufRead, BufReader, Read, Write};
use crossterm::style::Stylize;

pub fn clip(args:Vec<String>) {
    let file_path = &args[0];

    let current_dir = env::current_dir().expect("Failed to get current directory!");

    let path = PathBuf::from(file_path);

    let absolute_path = current_dir.join(path);

    let absolute_path_str = absolute_path.to_str().expect("Failed to convert path to string!");

    println!("Absolute path: {}", absolute_path_str);

    let temp_dir = env::temp_dir();

    let temp_file = temp_dir.join("rush-clipboard");

    let mut file = File::create(&temp_file).expect("Failed to create temp file");

    file.write(absolute_path_str.as_bytes())
        .expect(&format!("{} Failed to copy this file", "Error".bold().red()));

    println!("{} file copied to the clipboard", "Success".bold().green());

}

pub fn paste(_args:Vec<String>)  {
    let file_string = read_to_string(env::temp_dir().join("rush-clipboard")).expect("Failed to read file.");
    let lines: Vec<&str> = file_string.lines().collect();
    let source_path= lines.last().unwrap();
    let destination_path = current_dir().unwrap().join(source_path.split('/').last().unwrap());

    match copy(source_path, destination_path.to_str().unwrap()) {
        Ok(_) =>  println!("{} File copied successfully!", "Success".bold().green()) ,
        Err(e) => println!("{} Failed to copy file", "Error".bold().red() ),
    };

}