# Build stage
FROM rust:latest AS builder

# Install dependencies
RUN apt update && apt install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

# Build
RUN mkdir /app
ADD . /app/
WORKDIR /app
RUN make clean && RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# Publish stage
FROM alpine:latest

RUN mkdir /app
WORKDIR /app/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cloud-kitchen /app/app
ENV env=prod
CMD ./app
