static HELP: &str = r#"Usage
  add, --add [args]
  delete, --delete [args]
  list, --list [args]
  get, --get [args]
  help, --help [args]"#;

#[inline(always)]
pub fn main() {
    println!("{}", HELP)
}