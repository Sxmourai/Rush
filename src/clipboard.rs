use std::{env, io};
use std::path::PathBuf;
use std::fs::{File, read, copy};
use std::io::{BufRead, BufReader, Write};

pub fn clip(args:Vec<String>) {
    // Assuming the file path is passed as the first command-line argument
    let file_path = &args[0];

    // Get the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory!");

    // Create a PathBuf from the provided file path
    let path = PathBuf::from(file_path);

    // Get the absolute path by joining the current directory and the file path
    let absolute_path = current_dir.join(path);

    // Convert the absolute path to a String
    let absolute_path_str = absolute_path.to_str().expect("Failed to convert path to string!");

    // Print the absolute path
    println!("Absolute path: {}", absolute_path_str);

    // Get the system's temporary directory
    let temp_dir = env::temp_dir();

    // Create a temporary file in the temporary directory
    let temp_file = temp_dir.join("rush-clipboard");

    // Open the file for writing
    let mut file = File::create(&temp_file).expect("Failed to create temp file");

    // Get the absolute path
    // let absolute_path = env::current_exe().expect("Failed to get current executable path");

    // Write the absolute path to the temporary file
    file.write_all(absolute_path.to_string_lossy().as_bytes())
        .expect("Failed to write to temp file");

    // Print the path of the temporary file
    println!("Temporary file path: {:?}", temp_file);

}

pub fn paste(_args:Vec<String>)  {
    // Define the source and destination paths
    let clipboard_history_file = File::open(env::temp_dir().join("rush-clipboard")).unwrap();
    let buffer_reader = BufReader::new(clipboard_history_file);

    // Read the lines of the buffer into a vector
    let lines: Vec<String> = buffer_reader.buffer().lines().map(|line| line.unwrap()).collect();
    println!("{:?}",lines);
    // get last line
    let mut source_path=lines.last().unwrap();

    println!("Source path is{}",source_path);

    let destination_path = ".";

    // Copy the file from the source path to the destination path
    match copy(source_path, destination_path) {
        Ok(_) =>  println!("File copied successfully!") ,
        Err(e) => println!("Failed to copy file: {}", e),
    };

}