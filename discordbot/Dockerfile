FROM rust:1.83-bookworm

WORKDIR /app
COPY . .

RUN apt install openssl
RUN cargo build --release

CMD ["./target/release/crab_bot"]
