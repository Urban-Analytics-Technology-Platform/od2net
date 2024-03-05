FROM rust:latest as builder
WORKDIR /app
# Build the Rust tool
COPY Cargo.lock Cargo.toml .
COPY od2net ./od2net
COPY lts ./lts
COPY wasm-od2net ./wasm-od2net
RUN cargo build --release

# Build tippecanoe
# TODO Pin to a release?
RUN git clone https://github.com/felt/tippecanoe.git
RUN cd tippecanoe && make -j

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/od2net /usr/local/bin/od2net
COPY --from=builder /app/tippecanoe/tippecanoe /usr/local/bin/tippecanoe
# Need a dynamic library
RUN apt-get update
RUN apt-get install libsqlite3-0
ENTRYPOINT ["/usr/local/bin/od2net"]
