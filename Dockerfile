# Allow to select base image to test various options
ARG BASE_IMAGE=rust:latest
FROM ${BASE_IMAGE} AS builder

WORKDIR /app

# Build the Rust tool
COPY Cargo.lock Cargo.toml .
COPY od2net ./od2net
COPY lts ./lts
COPY wasm-od2net ./wasm-od2net

RUN cargo build --release

# Build the Tippecanoe
RUN git clone --depth=1 https://github.com/felt/tippecanoe.git \
    && cd tippecanoe \
    && make -j

# Runner Image
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/od2net /usr/local/bin/od2net
COPY --from=builder /app/tippecanoe/tippecanoe /usr/local/bin/tippecanoe

# Need a dynamic library
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends \
        libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/usr/local/bin/od2net"]
