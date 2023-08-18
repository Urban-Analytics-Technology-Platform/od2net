FROM rust:latest as builder
WORKDIR /app
COPY Cargo.lock Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/aggregate_routes /usr/local/bin/aggregate_routes
ENTRYPOINT ["/usr/local/bin/aggregate_routes"]
