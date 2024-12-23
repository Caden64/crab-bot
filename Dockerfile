# Use an official Rust runtime as a parent image
FROM rust:1.76-buster as builder

# Creating layer for caching dependencies
WORKDIR /usr/src/app
# Copying and building actual code
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Final stage: lightweight layer
FROM debian:buster-slim as runner

RUN apt-get update && apt-get install -y libssl-dev
# Creating app directory
WORKDIR /usr/src/app

# Copying binary from builder stage to the final stage
COPY --from=builder /usr/src/app/config.toml config.toml
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/crab_bot crab-bot

# Running binary
CMD ["./crab-bot"]