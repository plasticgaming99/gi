use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::{env, fs};
pub fn help() {
    println!(
        r#"
profile [args(optional)]
    args(optional)
    - new(aliases: create, n)
        - they are used to create a new profile.
    - --help(alias: -h)
        - they are used to display helps.

"#
    )
}
pub fn read_profiles() {
    let home = env::home_dir().expect("error: take a home dir value");
    let rcpath = home.join(".giprc");
    let rcdisp: String = rcpath.display().to_string().replace(&home.display().to_string(), "~");
    // println!("{}", rc);
    if Path::is_file(&rcpath) {
        let contents = fs::read_to_string(rcpath).expect("Should have been able to read the file");
        println!("{}", contents)
    } else {
        let mut yn = String::new();
        println!("{} is not found.", &rcdisp);
        print!("do you want to create a .giprc file? (Y/n)");
        stdout().flush().unwrap();
        let _ = stdin().read_line(&mut yn);
        if yn.to_lowercase() == "y" || yn.to_lowercase() == "yes" || yn.trim().is_empty() {
            let _ = fs::File::create(&rcpath);
            println!(".giprc is created successfully");
            println!("path: {}", &rcdisp);
        }
    }
}
