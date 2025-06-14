use std::io::{self, Write};

pub fn delete_line(lines: i32) {
    for _ in 0..lines {
        write!(io::stdout(), "\x1B[2K\x1B[1A").unwrap();
    };
}