FROM rust:latest as builder

RUN apt-get update && apt-get install -y musl-tools
ENV USER root
WORKDIR /pawn-pal
COPY . .
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /pawn-pal/target/x86_64-unknown-linux-musl/release/pawn-pal /pawn-pal
CMD ["./pawn-pal"]
