use byteorder::{BigEndian, WriteBytesExt};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::collections::HashMap;
use std::time::SystemTime;
use std::{fs, process};

fn help() {
    println!("gi run [arg]");
    println!("  this command runs a specified secret key.");
    println!("  [arg]: you need to type a profile name which you want to get code.");
    println!("         you also can use --help or help to output this help message. ");
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
    println!("HOTP");
    let digit: u32 = 6;
    let buf: &[u8] = &write_counter(count);
    type HmacSha1 = Hmac<Sha1>;

    let mut mac = HmacSha1::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(buf);
    let code_bytes = mac.finalize().into_bytes();
    let code = trunc(&code_bytes, digit.try_into().unwrap());
    let code = format!("{}", &code);
    loop {
        // code = format!("{}0", &code);
        println!("{}", code);
        if code.chars().count() == digit.try_into().unwrap() {
            return code;
        }
    }
}

fn totp(key: &[u8]) -> String {
    println!("TOTP");
    println!("{:?}", key);
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
    println!("{}", count.to_string());
    return hotp(key, count);
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

fn calculation(args: Vec<&str>) {
    let key: &[u8] = &base32(args[1].to_string());
    let key = totp(key);
    println!("{}", key);
}

fn output(arg: &str, path: String) {
    let profiles: String = fs::read_to_string(path).expect("");
    let profiles: Vec<&str> = profiles.lines().collect::<Vec<_>>();
    let profile: Option<&str> = profiles.iter().find(|s| s.split_whitespace().next() == Some(arg)).map(|v| &**v);
    match profile {
        Some(p) => {
            println!("Gi found the profile: {}", p);
            calculation(p.split_whitespace().collect());
        }
        None => {
            println!("Gi could not find the profile: {}", arg);
            process::exit(0);
        }
    }
}

pub fn main(args: Vec<String>, path: String) {
    if args.len() == 3 {
        match args[2].as_str() {
            "--help" | "-h" => help(),
            arg => output(arg, path),
        }
    }
}
