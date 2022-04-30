# syntax=docker/dockerfile:1
FROM rust as builder
WORKDIR /usr/src/wlogger
COPY . . 

RUN rustup override set nightly
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y && apt-get install libpq-dev -y && apt-get install curl -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/wlogger /usr/local/bin/wlogger
CMD ["wlogger"]