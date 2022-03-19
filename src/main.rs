extern crate rustc_serialize;
use std::process;
use regex::Regex;
use snailquote::unescape;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use colored::*;

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

fn main() {
    let encode_file = "morse-code-decode.json";
    let mut file = match File::open(encode_file) {
        Ok(ok) => ok,
        Err(_) => {
            println!("Error: Could not find encode file: {}", encode_file);
            process::exit(1);
        }
    };
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json = match Json::from_str(&data)  {
        Ok(ok) => ok,
        Err(_) => {
            println!("Error: Encode file is not valid");
            process::exit(1);
        }
    };
    
    loop{
        println!("Input:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error: Failed to read line");
        let clean_input = Regex::new(r"\s+").unwrap().replace_all(&input, " ").to_string();
        let clean_input_len = clean_input.chars().count();
        let mut vec = vec![String::new(); clean_input_len];
        let clean_input_slice = utf8_slice(&clean_input, 0, clean_input_len - 1).unwrap();
        let mut success = true; 
        for (i, c) in  clean_input_slice.chars().enumerate() {
            //println!("char: {:?}", c);
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
            //print!("{}", json.find_path(&[&c.to_string()]).unwrap());
        }
        if success {
            let joined = vec.join(" ");
            println!("\nOutput: {}", joined.green());
        }
    }
}
