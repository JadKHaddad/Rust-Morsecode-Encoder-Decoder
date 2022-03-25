# build: docker build -t rust-morse:1.0 .
# run: docker run --rm -it rust-morse:1.0 [OPTIONS]

FROM rust:1.59.0-slim-buster AS builder

RUN rustup target add x86_64-unknown-linux-musl

COPY main /home/app/main
COPY OpenFaaS /home/app/OpenFaaS

WORKDIR /home/app/main
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.14 as runner

COPY OpenFaaS /home/app/OpenFaaS
COPY --from=builder /home/app/main/target/x86_64-unknown-linux-musl/release/morsecode_encoder_decoder /home/app/main/morsecode_encoder_decoder

WORKDIR /home/app/main

ENTRYPOINT [ "/home/app/main/morsecode_encoder_decoder" ]