use std::{env, fs};

mod commands;

fn main() {
    let version: &str = "1.0.0";
    let foundstr: &str;
    let home_dir: String = env::home_dir().expect("").display().to_string();
    let path = format!("{}/.gi", home_dir);
    match fs::exists(&path) {
        Ok(true) => foundstr = ".gi found",
        Ok(false) => foundstr = ".gi not found",
        Err(_) => foundstr = ".gi Error"
    }
    println!("gi, Get Integer\nversion {}, {}", version, foundstr);

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match args[1].as_str() {
            "list" | "--list" => commands::list::main(args, path),
            "get" | "--get" => commands::get::main(args, path),
            "add" | "--add" => commands::add::main(args, path),
            "delete" | "--delete" => commands::delete::main(args, path),
            "help" | "--help" => commands::help::main(),
            _ => commands::help::main(),
        }
    }
}
