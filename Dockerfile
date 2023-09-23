FROM rust:latest as builder
WORKDIR /app
# Build the Rust tool
COPY Cargo.lock Cargo.toml .
COPY aggregate_routes ./aggregate_routes
COPY lts ./lts
RUN cargo build --release

# Build tippecanoe
# TODO Pin to a release?
RUN git clone https://github.com/felt/tippecanoe.git
RUN cd tippecanoe && make -j

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/aggregate_routes /usr/local/bin/aggregate_routes
COPY --from=builder /app/tippecanoe/tippecanoe /usr/local/bin/tippecanoe
# Need a dynamic library
RUN apt-get update
RUN apt-get install libsqlite3-0
ENTRYPOINT ["/usr/local/bin/aggregate_routes"]
