use std::{fs, io::ErrorKind, process};

fn help() {
    println!("gi delete [name]");
    println!("  this command deletes a specified secret key");
    println!("  [name]: you need to ")
}

fn search(name: String, contents: String) -> Result<Vec<String>, ErrorKind> {
    //        ::<>  ::<>
    //turbofish ::<>  ::<>
    //        ::<>  ::<>
    let mut names = contents.lines().collect::<Vec<&str>>();
    for i in 0..names.len() {
        if names[i].split_whitespace().collect::<Vec<&str>>().get(0).unwrap().trim().to_string() == name.to_string() {
            println!("{} was found", &name);
            names.remove(i);
            return Ok(names.iter().map(|&x| String::from(x)).collect());
        }
    }
    return Err(ErrorKind::NotFound);
}

fn delete(name: String, path: String) {
    let contents: String = match fs::read_to_string(&path) {
        Ok(str) => str,
        Err(_) => {
            println!("could not read ~/.gi!");
            process::exit(0);
        }
    };
    match search(name.to_string(), contents) {
        Ok(list) => {
            let mut content = String::new();
            if list.len() > 0 {
                content = list[0].clone();
                for i in 1..list.len() {
                    content = format!("{}\n{}", &content.trim(), list[i]);
                }
            }
            match fs::write(&path, content) {
                Ok(_) => println!("{} was deleted", &name),
                Err(_) => {
                    println!("{} could not be deleted", &name);
                    process::exit(0);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            process::exit(0);
        }
    }
}

pub fn main(args: Vec<String>, path: String) {
    if args.len() == 3 {
        match args[2].as_str() {
            "--help" | "-h" => help(),
            name => delete(name.to_string(), path),
        }
    }
}
