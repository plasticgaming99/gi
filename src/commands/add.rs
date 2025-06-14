use std::{
    fs,
    io::{self, Write},
    process,
};

fn help() {
    println!("gi add [name] [key]");
    println!("  this command adds a pair of a name and a key.");
    println!("  [name] and [key]: you need to type a profile name and secret key.");
    println!("gi add");
    println!("  this command adds a pair of a name and a key.");
    println!("  you don't need name and key if you want guided style.");
}

fn check_name(name: String, path: String) -> io::Result<()> {
    match fs::read_to_string(path) {
        Ok(str) => {
            println!("{}", str);
            let pairs: Vec<&str> = str.split("/n").collect();
            let mut names: Vec<&str> = Vec::new();
            for i in pairs {
                println!("{}", i);
                names.push(i.split_whitespace().take(1).next().unwrap());
            }
            for i in names {
                if i == name {
                    println!("{} is found. Please choose another name or delete the {}.", name, name);
                    return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Name already exists"));
                } else {
                    println!("Ok");
                }
            }
            return Ok(());
        }
        Err(err) => {
            println!("{}", err);
            Err(err)
        }
    }
}

fn make(name: String, key: String, path: String) {
    println!("name: {}", &name);
    println!("key: {}", &key);
    match check_name(name.clone().trim().to_string(), path.clone().trim().to_string()) {
        Ok(()) => {
            fs::write(&path, format!("{}{} {}\n", fs::read_to_string(path.clone()).unwrap().to_string(), &name.trim(), &key.trim())).expect("write error!");
            println!("{} was written!", name.trim())
        }
        Err(_) => {
            // println!("{} is found", name.trim());
            process::exit(0)
        }
    }
}
fn make_guide(path: String) {
    let mut name: String = String::new();
    let mut key: String = String::new();
    print!("name:");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut name);
    match check_name(name.clone().trim().to_string(), path.clone().trim().to_string()) {
        Ok(()) => {
            println!("{} is not found", name.trim());
            print!("key:");
            let _ = io::stdout().flush();
            let _ = io::stdin().read_line(&mut key);
            let content = fs::read_to_string(path.clone()).unwrap().to_string();
            fs::write(&path, format!("{}\n {} {}", &content.trim(), &name.trim(), &key.trim())).expect("write error!");
            println!("{} was written!", name.trim());
        }
        Err(_) => {
            // println!("{} is found", name.trim());
            process::exit(0)
        }
    }
}

pub fn main(args: Vec<String>, path: String) {
    if args.len() == 4 {
        make(args[2].clone(), args[3].clone(), path);
    } else if args.len() == 3 {
        match args[2].as_str() {
            "--help" | "-h" => help(),
            _ => {}
        }
    } else if args.len() == 2 {
        make_guide(path);
    }
}
