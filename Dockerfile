FROM rust:1.59.0 as builder

RUN mkdir /usr/src/rustwebservice
WORKDIR /usr/src/rustwebservice
COPY . .

RUN rustup default nightly
RUN cargo build --release

EXPOSE 8000

FROM gcr.io/distroless/cc-debian11


COPY --from=builder /home/kali/Desktop/Shield_website/DataGit/ServerActixPsql/target/debug/demo /usr/src/rustwebservice/

WORKDIR /usr/src/rustwebservice

EXPOSE 8000

CMD ["./demo"]