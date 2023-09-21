FROM rust:latest as builder
WORKDIR /app
COPY Cargo.lock Cargo.toml .
COPY aggregate_routes ./aggregate_routes
COPY lts ./lts
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/aggregate_routes /usr/local/bin/aggregate_routes
ENTRYPOINT ["/usr/local/bin/aggregate_routes"]
