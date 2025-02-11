# Build container
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
# Telepítsd a cmake-t és egyéb szükséges csomagokat
RUN apt-get update && \
    apt-get install -y mold && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /build

FROM chef AS planner
COPY . .
RUN RUSTFLAGS="-C target-cpu=native" cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /build/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN RUSTFLAGS="-C target-cpu=native" cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

# Final image
FROM debian:stable-slim

WORKDIR /app
COPY --from=builder /build/target/release/neptun-calendar-filter /app/neptun-calendar-filter
COPY --from=builder /build/files/                                /app/files/


# Running
EXPOSE 9876
CMD ["./neptun-calendar-filter"]