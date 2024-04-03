# Step 1: Compute a recipe file
FROM rust:1.85-bullseye AS planner
WORKDIR app
RUN cargo install cargo-chef
COPY .. .
RUN cargo chef prepare --recipe-path recipe.json

# Step 2: Cache project dependencies
FROM rust:1.85-bullseye AS cacher
WORKDIR app
RUN apt update && apt install -y libssl-dev build-essential cmake
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Step 3: Build the binary
FROM rust:1.85-bullseye AS builder
RUN apt update && apt install -y libssl-dev build-essential cmake
WORKDIR app
COPY .. .
# Copy over the cached dependencies from above
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# Step 4:
# Create a tiny output image.
# It only contains our final binary.
FROM rust:1.85-bullseye AS runtime
WORKDIR app
COPY --from=builder /app/target/release/moyu-server-emulator /usr/local/bin
ENTRYPOINT ["/usr/local/bin/moyu-server-emulator"]
