FROM rust:latest as build

WORKDIR /usr/src/shield_factory

COPY . .

RUN cargo install --path .

FROM alpine:latest

CMD ["cargo run"]
