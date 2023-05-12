use std::env;
use std::env::current_dir;
use std::path::PathBuf;
use std::fs::{copy, File, read_to_string};
use std::io::Write;
use crossterm::style::Stylize;

pub fn clip(args:Vec<String>) {
    let file_path = &args[0];

    let current_dir = env::current_dir().expect("Failed to get current directory!");

    let path = PathBuf::from(file_path);

    let absolute_path = current_dir.join(path);

    let absolute_path_str = absolute_path.to_str().expect("Failed to convert path to string!");

    let temp_dir = env::temp_dir();

    let temp_file = temp_dir.join("rush-clipboard");

    let mut file = File::create(&temp_file).expect("Failed to create temp file");

    file.write(absolute_path_str.as_bytes())
        .expect(&format!("{} Failed to copy this file", "Error".bold().red()));

    println!("{}, copied {} to the clipboard", "Success".bold().green(), absolute_path_str.bold());

}

pub fn paste(args:Vec<String>)  {
    let file_string = read_to_string(env::temp_dir().join("rush-clipboard")).expect(format!("{}: Nothing to paste", "Error".bold().red()).as_str());
    let lines: Vec<&str> = file_string.lines().collect();
    let source_abs_path= lines.last().unwrap();
    // If the user specifies a new name for the pasted file
    let dest_name = if args.len() > 0 {args[0].as_str()} else {source_abs_path.split('/').last().unwrap()};
    let destination_path = current_dir().unwrap().join(dest_name);

    match copy(source_abs_path, destination_path.to_str().unwrap()) {
        Ok(_) =>  println!("{} File copied successfully!", "Success".bold().green()) ,
        Err(e) => println!("{}: Failed to copy {} to {:?} ({})", "Error".bold().red(), source_abs_path, destination_path, e),
    };

}

pub fn history() {
    let file_string = read_to_string(env::temp_dir().join("rush-clipboard")).expect("Failed to read file.");
    let lines: Vec<&str> = file_string.lines().collect();
    println!("{} stores the following:","Rush - Clipboard".bold().green());
    for line in lines {
        println!("{}",line);
    }
}