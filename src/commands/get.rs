use byteorder::{BigEndian, WriteBytesExt};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::collections::HashMap;
use std::process::exit;
use std::thread::sleep;
use std::time::{self, SystemTime};
use std::{fs, process, str, vec};
use std::fmt::Write;

use crate::commands::delline::{delete_line};

static HELP: &str = r#"gi get [profiles, args]
  this subcommand runs a specified secret key.
  --all:
    print all the profiles.
  --totp (DEFAULT):
    print TOTP.
  --all-otp:
    print both.
  --update:
    update when code changed.
  --help: 
    output this help message."#;

#[inline(always)]
fn help() {
    println!("{}", HELP);
}

fn trunc(data: &[u8], digit: u32) -> u32 {
    let offset = (data[data.len() - 1] & 0x0f) as usize; // 0 <= offset <= 15
                                                         // Ensure offset is within bounds and there are at least 4 bytes remaining
    if offset + 4 > data.len() {
        // Handle out-of-bounds access, e.g., return an error or a default value
        // Here, we panic for simplicity, but a better approach would be to return a Result
        panic!("Offset out of bounds");
    }
    // Read 4 bytes as a big-endian u32
    let mut code = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);
    // Get last 31 bits
    code &= 0x7fffffff;
    // Calculate 10^DIGIT
    let power_of_ten = 10u32.pow(digit);
    return code % power_of_ten;
}

fn write_counter(counter: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(8);

    // Write the high 32 bits (counter / 2^32)
    let high = (counter >> 32) as u32;
    buffer.write_u32::<BigEndian>(high).expect("Failed to write high part");

    // Write the low 32 bits (counter % 2^32)
    let low = (counter & 0xFFFFFFFF) as u32;
    buffer.write_u32::<BigEndian>(low).expect("Failed to write low part");

    return buffer;
}

fn hotp(key: &[u8], count: u64) -> String {
    let digit: u32 = 6;
    let buf: &[u8] = &write_counter(count);
    type HmacSha1 = Hmac<Sha1>;

    let mut mac = HmacSha1::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(buf);
    let code_bytes = mac.finalize().into_bytes();
    let code = trunc(&code_bytes, digit.try_into().unwrap());
    let mut code_str = format!("{}", code);

    while code_str.chars().count() < digit.try_into().unwrap() {
        code_str = format!("0{}", code_str);
    }

    return code_str;
}

fn totp(key: &[u8]) -> String {
    let step: f64 = 30.0;
    let start: f64 = 0.0;
    let unixtime: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            println!("could not get UNIX TIME!");
            process::exit(0);
        }
    };
    let count: u64 = ((unixtime as f64 - start) / step).floor() as u64;
    let mut returnstr = String::new();
    write!(&mut returnstr, "{}", hotp(key, count)).unwrap();
    return returnstr
}

fn base32(input: String) -> Vec<u8> {
    // Base32の文字セット（RFC 4648準拠）
    const BASE32_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    // Base32文字セットを使って逆引きマップを作成
    let mut char_to_value = HashMap::new();
    for (i, c) in BASE32_ALPHABET.chars().enumerate() {
        char_to_value.insert(c, i as u8);
    }

    // 入力文字列を大文字に変換（Base32は大文字小文字を区別しない）
    let input = input.to_uppercase();

    // デコードされたバイト列を格納するためのベクタ
    let mut decoded_bytes = Vec::new();

    // 5ビットごとにデコードするためのバッファ
    let mut buffer = 0u32;
    let mut bits_in_buffer = 0;

    for c in input.chars() {
        // パディング文字（=）は無視する
        if c == '=' {
            continue;
        }

        // 入力文字がBase32文字セットに含まれているかチェック
        let value = match char_to_value.get(&c) {
            Some(&v) => v as u32,
            None => todo!(), //return Err(format!("Invalid Base32 character: {}", c)),
        };

        // 5ビットをバッファに追加
        buffer = (buffer << 5) | value;
        bits_in_buffer += 5;

        // バッファに8ビット以上たまったら1バイト取り出す
        if bits_in_buffer >= 8 {
            bits_in_buffer -= 8;
            let byte = (buffer >> bits_in_buffer) as u8;
            decoded_bytes.push(byte);
        }
    }

    return decoded_bytes;
}

fn calculate(args: Vec<&str>) -> String {
    let key: &[u8] = &base32(args[1].to_string());
    let key = totp(key);
    return key;
}

fn output_totp(arg: &str, path: &str) {
    let profiles: String = fs::read_to_string(path).expect("");
    let profiles: Vec<&str> = profiles.lines().collect::<Vec<_>>();
    let profile: Option<&str> = profiles.iter().find(|s| s.split_whitespace().next() == Some(arg)).map(|v| &**v);
    let mut buf = String::new();
    match profile {
        Some(p) => {
            writeln!(&mut buf, "Profile {}", p).unwrap();
            let st = calculate(p.split_whitespace().collect());
            let st = st.as_str();
            writeln!(&mut buf, "  TOTP: {}", st).unwrap();
        }
        None => {
            writeln!(&mut buf, "Profile {} Not found", arg).unwrap();
        }
    }
    if buf != "" {
        print!("{}", buf);
    }
}

/*fn output_hotp(arg: &str, path: &str) {
    let profiles: String = fs::read_to_string(path).expect("");
    let profiles: Vec<&str> = profiles.lines().collect::<Vec<_>>();
    let profile: Option<&str> = profiles.iter().find(|s| s.split_whitespace().next() == Some(arg)).map(|v| &**v);
    let mut buf = String::new();
    match profile {
        Some(p) => {
            writeln!(&mut buf, "Profile {}", p).unwrap();
            let st = calculate(p.split_whitespace().collect());
            let st = st.as_str();
            writeln!(&mut buf, "  HOTP: {}", st).unwrap();
        }
        None => {
            writeln!(&mut buf, "Profile {} Not found", arg).unwrap();
        }
    }
    if buf != "" {
        print!("{}", buf);
    }
}*/

pub fn main(args: Vec<String>, path: String) {
    struct ArgFlag {
        all: bool,
        totp: bool,
        //hotp: bool,
        update: bool,
        help: bool,
    }

    let mut arg_flags = ArgFlag{
        all: false,
        totp: false,
        //hotp: false,
        update: false,
        help: false,
    };

    let mut profiles = vec![];

    if args.len() >= 3 {
        for arg in &args[2..] {
        match arg.as_str() {
            "--all" => {arg_flags.all = true}
            "--totp" => {arg_flags.totp = true},
            "--update" => {arg_flags.update = true}
            "--help" | "-h" => {arg_flags.help = true},
            arg => profiles.push(arg),
        }
        }
    };

    if arg_flags.help {
        help();
        exit(1);
    };

    // default
    if !arg_flags.totp {
        arg_flags.totp = true;
    }

    if arg_flags.totp {
        loop {
            for prof in &profiles {
                output_totp(prof, &path);
            }
            if !arg_flags.update {
                break;
            }
            delete_line(2*profiles.len() as i32);
            let mut key: u64;
            let unixtime: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => n.as_secs(),
                Err(_) => {
                    println!("could not get UNIX TIME!");
                    process::exit(0);
                }
            };
            let step: f64 = 30.0;
            let count: u64 = ((unixtime as f64) / step).floor() as u64;
            key = count;
            loop {
                let unixtime: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(n) => n.as_secs(),
                    Err(_) => {
                        println!("could not get UNIX TIME!");
                        process::exit(0);
                    }
                };
                let count: u64 = ((unixtime as f64) / step).floor() as u64;
                sleep(time::Duration::from_millis(200));

                if key != count {
                    break;
                }
                key = count;
            }
        }
    };

    /*if arg_flags.hotp {
        for prof in &profiles {
            output_hotp(prof, &path)
        }
    }*/
}
