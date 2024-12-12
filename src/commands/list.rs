use std::fs;

pub fn main(path: String) {
    let content: String = fs::read_to_string(&path).expect("could not read the .gi !");
    let list: Vec<&str> = content.lines().collect::<Vec<_>>();
    println!("list:");
    for i in 0..list.len() {
        println!("{}", &list[i]);
    }
}
pub fn help() {}
