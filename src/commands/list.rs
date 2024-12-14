use std::fs;

fn help() {
    println!("gi list");
    println!("  this command shows you configured pairs of a name and a key");
}

fn list(path: String) {
    let content: String = fs::read_to_string(&path).expect("could not read the .gi !");
    let list: Vec<&str> = content.lines().collect::<Vec<_>>();
    println!("list:");
    for i in 0..list.len() {
        println!("{}", &list[i]);
    }
}

pub fn main(args: Vec<String>, path: String) {
    if args.len() == 3 {
        match args[2].as_str() {
            "--help" | "-h" => help(),
            _ => {},
        }
    } else if args.len() == 2 {
        list(path);
    }
}
