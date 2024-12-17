FROM rust:1.83-slim

WORKDIR /app
COPY . .

RUN cargo build --release

EXPOSE 8080
CMD ["./target/release/botapi"]
