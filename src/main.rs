use std::{
    env, fs,
    io::{stdin, stdout},
    process::exit,
};
mod commands;

fn main() {
    let version: &str = "1.0.0";
    println!("gi, Get Integer");
    println!("version {}", version);
    let home_dir: String = env::home_dir().expect("").display().to_string();
    let path = format!("{}/.gi", home_dir);

    match fs::exists(&path) {
        Ok(true) => println!(".gi was found"),
        Ok(false) => {
            println!(".gi was not found. please make a ~/.gi");
            print!("would like to make it?: ");
            let mut make = String::new();
            let _ = stdin().read_line(&mut make);
            match make.to_lowercase().as_str().trim() {
                "y" | "yes" => {
                    fs::File::create(path).unwrap();
                    println!(".gi was created successfully.");
                }
                _ => exit(0),
            }
            exit(0);
        }
        Err(_) => {}
    }
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match args[1].as_str() {
            "list" => commands::list::main(args, path),
            "run" => commands::run::main(args, path),
            "add" => commands::add::main(args, path),
            "delete" => commands::delete::main(args, path),
            _ => {}
        }
    }
}
