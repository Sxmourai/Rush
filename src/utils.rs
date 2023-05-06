pub fn error(message:&str) -> ! {
    println!("{}",message);
    std::process::exit(1);
}