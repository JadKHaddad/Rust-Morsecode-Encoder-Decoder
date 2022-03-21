# Rust-Morsecode-Encoder-Decoder

```sh
cd main
cargo run --
```
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
## OpenFaaS

Edit **OpenFaaS/morse-function.yml**

```sh
version: 1.0
provider:
  name: openfaas
  gateway: <your-openfaas-gateway>
functions:
  morse-function:
    lang: rust
    handler: ./function
    image: <your-container-registry>/morse-function:latest
```

Deploy
```sh
cd OpenFaaS

faas-cli up -f morse-function.yml
```
Try it
```sh
{"encode": true, "decode": false, "input": "serverless"}

# encode == true => encode
# encode == true && decode == true => encode
# encode == false && decode == true => decode
# encode == false && decode == false => Error
```