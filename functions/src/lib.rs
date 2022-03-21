extern crate rustc_serialize;
use regex::Regex;
use rustc_serialize::json::Json;
use snailquote::unescape;
use std::fs::File;
use std::io::Read;

pub enum FileStatus {
    NotFound,
    NotValid
}

pub fn json_from_file(path: &str) -> Result<Json, FileStatus> {
    let mut file = match File::open(path) {
        Ok(ok) => ok,
        Err(_) => {
            return Err(FileStatus::NotFound);
        }
    };
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json = match Json::from_str(&data) {
        Ok(ok) => ok,
        Err(_) => {
            return Err(FileStatus::NotValid);
        }
    };
    return Ok(json);
}

pub fn utf8_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    let mut iter = s
        .char_indices()
        .map(|(pos, _)| pos)
        .chain(Some(s.len()))
        .skip(start)
        .peekable();
    let start_pos = *iter.peek()?;
    for _ in start..end {
        iter.next();
    }
    Some(&s[start_pos..*iter.peek()?])
}

pub fn encode(input: &str, json: &Json) -> Result<String, String> {
    let clean_input = Regex::new(r"\s+")
        .unwrap()
        .replace_all(&input, " ")
        .to_string();
    let clean_input_len = clean_input.chars().count();
    let mut vec = vec![String::new(); clean_input_len];
    let clean_input_slice = utf8_slice(&clean_input, 0, clean_input_len).unwrap();
    for (i, c) in clean_input_slice.chars().enumerate() {
        if i == clean_input_len - 1 && c == ' ' {
            break;
        }
        let code_char = unescape(
            &match json.find_path(&[&c.to_string()]) {
                Some(ok) => ok,
                None => {
                    return Err(c.to_string());
                }
            }
            .to_string(),
        )
        .unwrap();
        vec[i] = code_char;
    }
    let joined = vec.join(" ");
    return Ok(joined);
}

pub fn decode(input: &str, json: &Json) -> Result<String, String> {
    let clean_input = Regex::new(r"\s+")
        .unwrap()
        .replace_all(&input, " ")
        .to_string();
    let clean_input = Regex::new(r"/+")
        .unwrap()
        .replace_all(&clean_input, "/")
        .to_string();
    let split = clean_input.split(" ");
    let words: Vec<&str> = split.collect();
    let mut vec = vec![String::new(); words.len()];
    for i in 0..words.len() {
        let code_char = unescape(
            &match json.find_path(&[&words[i].to_string()]) {
                Some(ok) => ok,
                None => {
                    return Err(words[i].to_string());
                }
            }
            .to_string(),
        )
        .unwrap();
        vec[i] = code_char;
    }
    let joined = vec.join("");
    return Ok(joined);
}