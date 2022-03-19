extern crate rustc_serialize;
use std::process;
use regex::Regex;
use snailquote::unescape;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use colored::*;
use clap::{Arg, App};

static ENCODE_FILE: &str = "morse-code-encode.json";
static DECODE_FILE: &str = "morse-code-decode.json";

pub struct CustomError {
    
}

pub fn json_from_file(path: &str) -> Json{
    let mut file = match File::open(path) {
        Ok(ok) => ok,
        Err(_) => {
            println!("Error: Could not find file: {}", path);
            process::exit(1);
        }
    };
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json = match Json::from_str(&data)  {
        Ok(ok) => ok,
        Err(_) => {
            println!("Error: Encode file is not valid: {}", path);
            process::exit(1);
        }
    };
    return json;
}

pub fn utf8_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    let mut iter = s.char_indices()
        .map(|(pos, _)| pos)
        .chain(Some(s.len()))
        .skip(start)
        .peekable();
    let start_pos = *iter.peek()?;
    for _ in start..end { iter.next(); }
    Some(&s[start_pos..*iter.peek()?])
}

pub fn encode(input: &str) -> Result<String, CustomError> {
    let json = json_from_file(ENCODE_FILE);
    let clean_input = Regex::new(r"\s+").unwrap().replace_all(&input, " ").to_string();
    let clean_input_len = clean_input.chars().count();
    let mut vec = vec![String::new(); clean_input_len];
    let clean_input_slice = utf8_slice(&clean_input, 0, clean_input_len).unwrap();
    let mut success = true; 
    for (i, c) in  clean_input_slice.chars().enumerate() {
        if c == '\n' {
            break;
        }
        let code_char = unescape(& match json.find_path(&[&c.to_string()]){
            Some(ok) => ok,
            None => {
                println!("The fuck is this {} ? Try again", c.to_string().red());
                success = false;
                break;
            }
        }.to_string()).unwrap();
        vec[i] = code_char;
    }
    if success {
        let joined = vec.join(" ");
        return Ok(joined);
    }
    Err(CustomError{})
}

pub fn decode(input: &str) -> Result<String, CustomError> {
    let json = json_from_file(DECODE_FILE);
    let clean_input = Regex::new(r"\s+").unwrap().replace_all(&input, " ").to_string();
    let clean_input = Regex::new(r"/+").unwrap().replace_all(&clean_input, "/").to_string();
    let split = clean_input.split(" ");
    let words: Vec<&str> = split.collect();
    let mut vec = vec![String::new(); words.len()];
    let mut success = true; 
    for i in 0..words.len(){
        let code_char = unescape(& match json.find_path(&[&words[i].to_string()]){
            Some(ok) => ok,
            None => {
                println!("The fuck is this {} ? Try again", words[i].to_string().red());
                success = false;
                break;
            }
        }.to_string()).unwrap();
        vec[i] = code_char;
    }
    if success {
        let joined = vec.join("");
        return Ok(joined);
    }
    Err(CustomError{})
}

pub fn print_not_valid_input(){
    println!("{} Use -h for more info", "Input is not valid.".red());
}

fn main() {
    let matches = App::new("Morsecode Encoder Decoder")
        .version("1.0")
        .arg(Arg::with_name("encode")
                 .short('e')
                 .long("encode")
                 .takes_value(true)
                 .help("Encode string. Usage: -e \"[string]\""))
        .arg(Arg::with_name("decode")
                 .short('d')
                 .long("decode")
                 .takes_value(true)
                 .help("Decode morsecode. Usage: -d \"[-. .. -.-. .]\""))
        .get_matches();

    let encode_val = matches.value_of("encode").unwrap_or("");
    let decode_val = matches.value_of("decode").unwrap_or("");
    let re = Regex::new(r"^\[.*?\]$").unwrap();
    if encode_val != "" {
        if re.is_match(encode_val){
            match encode(&encode_val[1..encode_val.chars().count() - 1]) {
                Ok(ok) => {
                    println!("{}", ok.green());
                    process::exit(1);
                },
                _ => process::exit(1)
            };
        }
        else{
            print_not_valid_input(); 
        }
    }
    else if decode_val != "" {
        if re.is_match(decode_val){
            match decode(&decode_val[1..decode_val.chars().count() - 1]) {
                Ok(ok) => {
                    println!("{}", ok.green());
                    process::exit(1);
                },
                _ => process::exit(1)
            };
        }
        else{
            print_not_valid_input();
        }
    }
    else {
        loop{
            println!("Input:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Error: Failed to read line");
            /*match encode(&input) {
                Ok(ok) => println!("\nOutput: {}", ok.green()),
                Err(_) => continue
            };
            */
            match decode(&input) {
                Ok(ok) => println!("\nOutput: {}", ok.green()),
                Err(_) => continue
            };
        }
    }
}