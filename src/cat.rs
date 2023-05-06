use std::io::{ErrorKind, Write};
use clap::Parser;
use crossterm::{*, event::*};
use crate::utils::*;

#[derive(Parser, Debug)]
struct Args {
    filename: String,
    #[arg(short, long, default_value_t = false)]
    raw: bool,
}

pub fn run(mut args:Vec<String>) {
    args.insert(0,"cat".to_string());
    let args = Args::parse_from(args);
    let result = match std::fs::read_to_string(&args.filename) {
        Ok(content) => content,
        Err(err) => error(&format!("Couldn't read {} because {}.", args.filename, match err.kind() {
            ErrorKind::NotFound=>"the file wasn't found",
            ErrorKind::PermissionDenied=>"you don't have permissions",
            ErrorKind::ConnectionRefused=>"the connection was refused",
            ErrorKind::ConnectionReset=>"the connection was reset",
            ErrorKind::ConnectionAborted=>"the connection was aborted",
            ErrorKind::NotConnected=>"you are not connected",
            ErrorKind::AddrInUse=>"the addres is in use",
            ErrorKind::AddrNotAvailable=>"the address is not available",
            ErrorKind::BrokenPipe=>"the pipe is broken",
            ErrorKind::AlreadyExists=>"the file already exists",
            ErrorKind::WouldBlock=>"the file would blocked",
            ErrorKind::InvalidInput=>"the file input was invalid",
            ErrorKind::InvalidData=>"the file data was invalid",
            ErrorKind::TimedOut=>"the file timed out",
            ErrorKind::WriteZero=>"write zero",
            ErrorKind::Interrupted=>"the process was interrupted",
            ErrorKind::Unsupported=>"the file is unsupported",
            ErrorKind::UnexpectedEof=>"the file has an unexpected EOF (End Of File)",
            ErrorKind::OutOfMemory=>"you ran out of memory",
            ErrorKind::Other=>"the error isn't supported",
            // ErrorKind::HostUnreachable => todo!(),           UNSTABLE
            // ErrorKind::NetworkUnreachable => todo!(),
            // ErrorKind::NetworkDown => todo!(),
            // ErrorKind::NotADirectory => todo!(),
            // ErrorKind::IsADirectory => todo!(),
            // ErrorKind::DirectoryNotEmpty => todo!(),
            // ErrorKind::ReadOnlyFilesystem => todo!(),
            // ErrorKind::FilesystemLoop => todo!(),
            // ErrorKind::StaleNetworkFileHandle => todo!(),
            // ErrorKind::StorageFull => todo!(),
            // ErrorKind::NotSeekable => todo!(),
            // ErrorKind::FilesystemQuotaExceeded => todo!(),
            // ErrorKind::FileTooLarge => todo!(),
            // ErrorKind::ResourceBusy => todo!(),
            // ErrorKind::ExecutableFileBusy => todo!(),
            // ErrorKind::Deadlock => todo!(),
            // ErrorKind::CrossesDevices => todo!(),
            // ErrorKind::TooManyLinks => todo!(),
            // ErrorKind::InvalidFilename => todo!(),
            // ErrorKind::ArgumentListTooLong => todo!(),
            _ => "unsupported error", 
        })),
    };
    if args.raw {println!("{}",result)}
    else {
        terminal::Clear(terminal::ClearType::All);
        let size = terminal::size().unwrap();
        let (cols, rows) = (size.0 as usize, size.1 as usize);
        let separator = "═".repeat(cols-1);
        print!("╔{}\n", separator);
        let splitted:Vec<&str> = result.split("\n").collect();
        if rows > splitted.len() {
            for line in splitted {
                print!("║ {}\n", line);
            }
        } else {
            let mut i = 0;
            for i in 0..rows {
                print!("║ {}\n", splitted[i]);
            }
            std::io::stdout().flush().unwrap();
            while i < splitted.len() {
                if event::read().unwrap() == Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)){
                    println!("║ {}", splitted[i]);
                    i += 1;
                }
            }
        };
        print!("╚{}\n", separator);
        std::io::stdout().flush().unwrap();
    }
}

