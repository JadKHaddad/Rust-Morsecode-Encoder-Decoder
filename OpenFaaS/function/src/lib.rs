extern crate rustc_serialize;
use regex::Regex;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use snailquote::unescape;
use std::fs::File;
use std::io::Read;

pub enum FileStatus {
    NotFound,
    NotValid,
}

pub enum Decision {
    ENCODE,
    DECODE,
}

pub fn decide(input: &str) -> Decision {
    let clean_input = clean_input(input);
    let split = clean_input.split(" ");
    let words: Vec<&str> = split.collect();
    // if all words consist of - and . => decode else encode
    let mut encode = false;
    let re = Regex::new(r"^[.-]+$").unwrap();
    let re_2 = Regex::new(r"^[/]+$").unwrap();
    for word in &words {
        if !re.is_match(word) {
            if !re_2.is_match(word) {
                encode = true;
                break;
            }
        }
    }
    if encode {
        return Decision::ENCODE;
    }
    return Decision::DECODE;
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

pub fn clean_input(input: &str) -> String {
    let clean_input = Regex::new(r"\s+")
        .unwrap()
        .replace_all(&input, " ")
        .to_string();
    return clean_input.trim().to_string();
}

pub fn encode(input: &str, json: &Json) -> Result<String, String> {
    let clean_input = clean_input(input);
    let clean_input = clean_input.trim();
    let clean_input_len = clean_input.len();
    let mut vec = vec![String::new(); clean_input_len];
    let clean_input_slice = utf8_slice(&clean_input, 0, clean_input_len).unwrap();
    for (i, c) in clean_input_slice.chars().enumerate() {
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
    let clean_input = clean_input(input);
    // remove duplicate char: /
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
    return Ok(joined.trim().to_string());
}

static BAD_REQUEST_MESSAGE: &str =
    "Bad request. Usage: {\"encode\": <bool>, \"decode\": <bool>, \"input\": <string>}";

#[derive(RustcEncodable)]
pub struct HttpResult {
    success: bool,
    output: String,
    message: String,
}

#[derive(RustcDecodable)]
pub struct HttpReuquest {
    encode: bool,
    decode: bool,
    input: String,
}

pub fn response_on_server_error() -> String {
    let response = HttpResult {
        success: false,
        output: String::new(),
        message: String::from("Server error"),
    };
    return json::encode(&response).unwrap();
}

pub fn response_on_failure(message: String) -> String {
    let response = HttpResult {
        success: false,
        output: String::new(),
        message: message,
    };
    return json::encode(&response).unwrap();
}

pub fn response_on_success(output: String, message: String) -> String {
    let response = HttpResult {
        success: true,
        output: output,
        message: message,
    };
    return json::encode(&response).unwrap();
}

pub fn handle(req: String, encode_file: &str, decode_file: &str) -> String {
    // parse json from string
    let request: HttpReuquest = match json::decode(&req) {
        Ok(ok) => ok,
        Err(_) => {
            return response_on_failure(BAD_REQUEST_MESSAGE.to_string());
        }
    };
    // request is valid
    let json_encode = match json_from_file(encode_file) {
        Ok(ok) => ok,
        Err(err) => match err {
            FileStatus::NotFound => {
                return response_on_server_error();
            }
            FileStatus::NotValid => {
                return response_on_server_error();
            }
        },
    };
    let json_decode = match json_from_file(decode_file) {
        Ok(ok) => ok,
        Err(err) => match err {
            FileStatus::NotFound => {
                return response_on_server_error();
            }
            FileStatus::NotValid => {
                return response_on_server_error();
            }
        },
    };

    if request.encode {
        match encode(&request.input, &json_encode) {
            Ok(ok) => {
                return response_on_success(ok, String::from("Encode"));
            }
            Err(err) => {
                return response_on_failure(format!("Could not understand this: [{}]", err));
            }
        };
    }
    if request.decode {
        match decode(&request.input, &json_decode) {
            Ok(ok) => {
                return response_on_success(ok, String::from("Decode"));
            }
            Err(err) => {
                return response_on_failure(format!("Could not understand this: [{}]", err));
            }
        };
    }
    return response_on_failure(BAD_REQUEST_MESSAGE.to_string());
}
