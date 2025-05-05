FROM rust:1.86.0-alpine AS chef
USER root
RUN apk add musl-dev
RUN cargo install cargo-chef
WORKDIR /app

# Step 1: Compute a recipe file
FROM chef AS planner
COPY .. .
RUN cargo chef prepare --recipe-path recipe.json

# Step 2: Cache project dependencies
FROM chef AS cacher
RUN apk add openssl-dev build-base cmake
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Step 3: Build the binary
FROM chef AS builder
RUN apk add openssl-libs-static
COPY .. .
# Copy over the cached dependencies from above
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# Step 4:
# Create a tiny output image.
# It only contains our final binary.
FROM alpine:3.21 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/moyu-server-emulator /usr/local/bin
ENTRYPOINT ["/usr/local/bin/moyu-server-emulator"]
