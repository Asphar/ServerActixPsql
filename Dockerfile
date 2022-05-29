FROM rust:latest as builder


RUN mkdir -p /usr/src/actix
WORKDIR /usr/src/actix
COPY . .
RUN cargo build --release


FROM rust:latest

WORKDIR /usr/src/actix

COPY --from=builder /usr/src/actix/target/release/demo /usr/src/actix/
COPY cert.pem cert.pem
COPY key.pem key.pem
COPY .env .env

EXPOSE 8043

CMD ["./demo"]