FROM rust:latest as builder
WORKDIR /app
# Build the Rust tool
COPY Cargo.lock Cargo.toml .
COPY od2net ./od2net
COPY lts ./lts
COPY wasm-od2net ./wasm-od2net
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/od2net /usr/local/bin/od2net
ENTRYPOINT ["/usr/local/bin/od2net"]
