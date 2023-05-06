use std::fs::write;
use std::env::var;
use std::path::Path;

pub fn clip(args:Vec<String>) {
    // let file = std::fs::File::open().unwrap();
    // let file = ;
    println!("{:?}", std::fs::canonicalize(&args[0]).unwrap().as_path().as_os_str());
    // let mut clip_path = "/tmp/clip";
    // let temp = var("TEMP").unwrap();
    // if cfg!(windows) {
    //     if var("TEMP").is_ok() {
    //         clip_path = temp.as_str();
    //     } else {
    //         clip_path = "C:\\WINDOWS\\TEMP";
    //     }
    // }
    // println!("{}", clip_path);
    // write(format!("{}/clip", clip_path), filename).unwrap();
}

pub fn paste(_args:Vec<String>) {
    let file = std::fs::read_dir("\\\\?\\B:\\Documents\\Code\\Projets\\Rush").unwrap();
    println!("{:?}", file);
}