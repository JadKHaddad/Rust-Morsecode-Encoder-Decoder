# clean before build: cargo clean
# build: docker build -t rust-morse:1.0 .
# run: docker run --rm -it rust-morse:1.0 [OPTIONS]

FROM rust:1.59.0-slim-buster

COPY . /home/app/
WORKDIR /home/app/
RUN cargo build --release
ENTRYPOINT [ "/home/app/target/release/morsecode_encoder_decoder" ]