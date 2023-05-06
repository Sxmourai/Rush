use std::io::Write;
use clap::Parser;
use crate::utils::error;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = ".")]
    path: String,
    #[arg(long, default_value_t = true)]
    raw: bool,
    #[arg(short, long, default_value_t = true)]
    size: bool,
    #[arg(short, long, default_value_t = true)]
    permissions: bool,
    #[arg(short, long, default_value_t = false)]
    recursive: bool,
}

pub fn run(mut args:Vec<String>) {
    args.insert(0,"ls".to_string());
    let args = Args::parse_from(args);
    if args.raw {
        // println!("{:?}",args);
        let content = get_dir(args.path, args.recursive);
        printdir(content, 0, args.size);
        std::io::stdout().flush().unwrap();
    }
}

fn printdir(dir:Vec<DirContent>, nested:u16, size:bool) {
    let mut i = 0;
    let len = dir.len();
    for ele in dir {
        let mut cline = String::new();
        if nested > 0 {
            if nested > 1 && i < len-1 {
                cline.insert_str(0,&format!("╚═{}", "╬═".repeat((nested-1).into())));
            }
            else if nested > 1 {
                cline.insert_str(0,&format!("╚═{}", "╩═".repeat(nested as usize-1)));
            }
            else if i < len-1 {
                cline.insert_str(0,&"╠═".repeat(nested.into()));
            } else{
                cline.insert_str(0,&"╚═".repeat(nested.into()));
            }
        }
        print!("{}{}\n",cline,ele.name);
        if let Some(recursed) = ele.content {
            if nested == 0 {
                printdir(*recursed, nested+1, size);
            }
        }
        // if args.permissions {cline.push_str(&format!("{}",s.metadata().unwrap().permissions().))}
        // if size {cline.push_str(&format!("{}ko",ele.len()/1000))}
        i+=1;
    }
}


fn get_dir(dir:String, recursive:bool) -> Vec<DirContent> {
    let mut content:Vec<DirContent> = Vec::new();
    let raw_content = match std::fs::read_dir(&dir) {
        Ok(r) => r,
        Err(e) => match e {
            _ => error(&format!("Error reading directory {}",dir)),
        }
    };
    for ele in raw_content {
        let ele = ele.unwrap();
        let name = ele.file_name().to_str().unwrap().to_string();
        content.push(if ele.metadata().unwrap().is_file() {DirContent{name, content:None}} else {
            DirContent{name, content:Some(Box::new(get_dir(format!("{}/{}",dir,ele.file_name().to_str().unwrap().to_owned() ),recursive)))}
        });
    }
    return content;
}

struct DirContent{
    name:String,
    content: Option<Box<Vec<DirContent>>>, 
}