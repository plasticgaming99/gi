use std::env;
use std::io::stdin;

mod commands;
fn main() {
    let mut args: Vec<String> = env::args().collect();
    // let mut input: String = String::new();
    // let _ = stdin().read_line(&mut input);

    if args.len() >= 2 {
        match args[1].as_str() {
            "profile" => {
                if args.len() == 3 {
                    match args[2].as_str() {
                        "--help" | "-h" => {
                            commands::profile::help();
                        }
                        "new" | "create" | "n" => {}
                        _ => {
                            eprintln!("not found this subcommand: {}", args[1]);
                            eprintln!(r#"tips: type "-h" or "--help""#);
                        }
                    }
                } else {
                    commands::profile::read_profiles();
                }
            }
            _ => {
                eprintln!("not found this subcommand: {}", args[1]);
                eprintln!(r#"tips: type "-h" or "--help""#);
            }
        }
    } else {
        eprintln!("env::args().collect(): error, need args");
    }
}
