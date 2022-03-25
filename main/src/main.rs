use clap::{Arg, Command};
use colored::*;
use regex::Regex;
use std::process;

extern crate function;

static ENCODE_FILE: &str = "../OpenFaaS/function/morse-code-encode.json";
static DECODE_FILE: &str = "../OpenFaaS/function/morse-code-decode.json";

pub fn print_not_valid_input() {
    println!("{} Use -h for help", "Input is not valid.".red());
}

fn main() {
    let matches = Command::new("Morsecode Encoder/Decoder")
        .arg_required_else_help(true)
        .version("1.0")
        .arg(
            Arg::new("encode")
                .short('e')
                .long("encode")
                .takes_value(true)
                .help("Encode string. Usage: -e \"[nice]\""),
        )
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .takes_value(true)
                .help("Decode morsecode. Usage: -d \"[-. .. -.-. .]\""),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Interactive session with dynamic input"),
        )
        .get_matches();

    let encode_val = matches.value_of("encode").unwrap_or("");
    let decode_val = matches.value_of("decode").unwrap_or("");
    let interactive = matches.is_present("interactive");
    let re = Regex::new(r"^\[.*?\]$").unwrap();
    let json_encode = match function::json_from_file(ENCODE_FILE) {
        Ok(ok) => ok,
        Err(err) => match err {
            function::FileStatus::NotFound => {
                println!("Error: Could not find file: {}", ENCODE_FILE);
                process::exit(1);
            }
            function::FileStatus::NotValid => {
                println!("Error: Encode file is not valid: {}", ENCODE_FILE);
                process::exit(1);
            }
        },
    };
    let json_decode = match function::json_from_file(DECODE_FILE) {
        Ok(ok) => ok,
        Err(err) => match err {
            function::FileStatus::NotFound => {
                println!("Error: Could not find file: {}", DECODE_FILE);
                process::exit(1);
            }
            function::FileStatus::NotValid => {
                println!("Error: Encode file is not valid: {}", DECODE_FILE);
                process::exit(1);
            }
        },
    };
    if encode_val != "" {
        if re.is_match(encode_val) {
            match function::encode(&encode_val[1..encode_val.len() - 1], &json_encode) {
                Ok(ok) => {
                    println!("{}", ok.green());
                    process::exit(0);
                }
                Err(err) => {
                    println!(
                        "The fuck is this {} ? Try again or use -h for help",
                        err.red()
                    );
                    process::exit(1);
                }
            };
        } else {
            print_not_valid_input();
        }
    } else if decode_val != "" {
        if re.is_match(decode_val) {
            match function::decode(&decode_val[1..decode_val.len() - 1], &json_decode) {
                Ok(ok) => {
                    println!("{}", ok.green());
                    process::exit(0);
                }
                Err(err) => {
                    println!(
                        "The fuck is this {} ? Try again or use -h for help",
                        err.red()
                    );
                    process::exit(1);
                }
            };
        } else {
            print_not_valid_input();
        }
    } else if interactive {
        loop {
            println!("Input:");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error: Failed to read line");
            match function::decide(&input) {
                function::Decision::ENCODE => match function::encode(&input, &json_encode) {
                    Ok(ok) => println!("\nEncode: {}", ok.green()),
                    Err(err) => {
                        println!(
                            "The fuck is this {} ? Try again or use -h for help",
                            err.red()
                        );
                        continue;
                    }
                },
                function::Decision::DECODE => match function::decode(&input, &json_decode) {
                    Ok(ok) => println!("\nDecode: {}", ok.green()),
                    Err(err) => {
                        println!(
                            "The fuck is this {} ? Try again or use -h for help",
                            err.red()
                        );
                        continue;
                    }
                },
            }
        }
    }
}
