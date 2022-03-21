use std::io::{self, Read};

extern crate function;

static ENCODE_FILE: &str = "/usr/bin/morse-code-encode.json";
static DECODE_FILE: &str = "/usr/bin/morse-code-decode.json";

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    println!("{}", function::handle(buffer, ENCODE_FILE, DECODE_FILE));
    Ok(())
}
