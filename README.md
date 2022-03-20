# Rust-Morsecode-Encoder-Decoder

```sh
Morsecode Encoder Decoder 1.0

USAGE:
    morsecode_encoder_decoder [OPTIONS]

OPTIONS:
    -d, --decode <decode>    Decode morsecode. Usage: -d "[-. .. -.-. .]"
    -e, --encode <encode>    Encode string. Usage: -e "[string]"
    -h, --help               Print help information
    -i, --interactive        Interactive session with dynamic input
    -V, --version            Print version information
```

## Docker

Build
```sh
docker build -t rust-morse:1.0 .
```
Run
```sh
docker run --rm -it rust-morse:1.0 [OPTIONS]
```