use std::{fs, process};
use std::collections::HashMap;
use std::time::SystemTime;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use byteorder::{BigEndian, WriteBytesExt};

fn help() {
    println!("gi run [arg]");
    println!("  [arg]: you need to type a profile name which you want to get code.");
    println!("         you also can use --help or help to output this help message. ");
}
fn trunc(data: &[u8], digit: u32) -> u32 {
    let offset: usize = (data[data.len() - 1] & 0x0f).into();
    let code = u32::from_be_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ]) & 0x7fffffff;

    code % 10u32.pow(digit)
}
fn write_counter(counter: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(8);

    // Write the high 32 bits (counter / 2^32)
    let high = (counter >> 32) as u32;
    buffer.write_u32::<BigEndian>(high).expect("Failed to write high part");

    // Write the low 32 bits (counter % 2^32)
    let low = (counter & 0xFFFFFFFF) as u32;
    buffer.write_u32::<BigEndian>(low).expect("Failed to write low part");

    buffer
}
fn hotp(key: &[u8], count: u64) -> String {
    println!("HOTP");
    let digit: i32 = 6;
    let buf: &[u8] = &write_counter(count);
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(buf);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let code = trunc(&code_bytes, digit.try_into().unwrap());
    let mut code = format!("{}", &code);
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
    let step: u64 = 30;
    let unixtime: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {println!("could not get UNIX TIME!"); process::exit(0);},
    };
    let count: u64 = unixtime/1000/step;
    return hotp(key, count);
}

fn base32table() -> HashMap<char, u8> {
    let mut table = HashMap::new();
    table.insert('A', 0); table.insert('J', 9); table.insert('S', 18); table.insert('3', 27);
    table.insert('B', 1); table.insert('K', 10); table.insert('T', 19); table.insert('4', 28);
    table.insert('C', 2); table.insert('L', 11); table.insert('U', 20); table.insert('5', 29);
    table.insert('D', 3); table.insert('M', 12); table.insert('V', 21); table.insert('6', 30);
    table.insert('E', 4); table.insert('N', 13); table.insert('W', 22); table.insert('7', 31);
    table.insert('F', 5); table.insert('O', 14); table.insert('X', 23);
    table.insert('G', 6); table.insert('P', 15); table.insert('Y', 24);
    table.insert('H', 7); table.insert('Q', 16); table.insert('Z', 25);
    table.insert('I', 8); table.insert('R', 17); table.insert('2', 26);
    table
}

fn base32(secret: String) -> Vec<u8> {
    let mut str = secret.to_uppercase();
    str.retain(|c| c.is_ascii_alphanumeric() && (c.is_digit(10) || c.is_ascii_alphabetic()));

    let padding_len = (8 - str.len() % 8) % 8;
    let mut str_padded = str.clone();
    str_padded.push_str(&"A".repeat(padding_len));
    let table = base32table();
    let mut data: Vec<u8> = Vec::with_capacity(str_padded.len() * 5 / 8);

    for chunk in str_padded.as_bytes().chunks(8) {
        let mut buf = vec![0u8; 5];
        let mut tmp: u32 = 0;

        let mut shift = 35;
        for &c in chunk.iter() {
            let value = *table.get(&(c as char)).unwrap_or(&0);
            let shift_clamped = shift.min(31);
            tmp |= (value as u32) << shift_clamped;
            shift -= 5;
        }

        // println!("tmp: {:032b}", tmp);  // Debugging the tmp value in binary

        buf[0] = (tmp >> 24) as u8;
        buf[1] = (tmp >> 16) as u8;
        buf[2] = (tmp >> 8) as u8;
        buf[3] = (tmp & 0xFF) as u8;
        buf[4] = (tmp >> 0) as u8;

        data.extend_from_slice(&buf);
    }
    return data;
}
fn calculation(args: Vec<&str>) {
    let key: &[u8] = &base32(args[1].to_string());
    let key = totp(key);
    println!("{}", key)
}

fn output(arg: &str, path: String) {
    let profiles: String = fs::read_to_string(path).expect("");
    let profiles: Vec<&str> = profiles.lines().collect::<Vec<_>>();
    let profile: Option<&str> = profiles.iter().find(|s| s.split_whitespace().next() == Some(arg)).map(|v| &**v);
    match profile {
        Some(p) => {
            println!("Gi found the profile: {}", p);
            calculation(p.split_whitespace().collect());
        },
        None => {
            println!("Gi could not find the profile: {}", arg);
            process::exit(0);
        }
    }
}

pub fn main(args: Vec<String>, path: String) {
    match args[2].as_str() {
        "--help" | "-h" => help(),
        arg => output(arg, path),
    }
}
