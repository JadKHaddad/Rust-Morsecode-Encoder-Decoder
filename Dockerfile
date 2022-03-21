# build: docker build -t rust-morse:1.0 .
# run: docker run --rm -it rust-morse:1.0 [OPTIONS]

FROM rust:1.59.0-slim-buster

COPY main /home/app/main
COPY functions /home/app/functions

WORKDIR /home/app/main
RUN cargo build --release
ENTRYPOINT [ "/home/app/main/target/release/morsecode_encoder_decoder" ]