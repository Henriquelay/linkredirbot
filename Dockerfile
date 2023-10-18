# Using the `rust-musl-builder` as base image, instead of 
# the official Rust toolchain
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json --target x86_64-unknown-linux-musl
# Build application
COPY . .
RUN cargo build --release --bin linkredirbot --target x86_64-unknown-linux-musl

# We do not need the Rust toolchain to run the binary!
FROM alpine AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/linkredirbot /usr/local/bin/
USER myuser

ENTRYPOINT ["linkredirbot"]
