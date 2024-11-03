# Use the official Rust image to build the project
FROM rust:1.82 AS builder

WORKDIR /app
COPY . .

# Build the project
RUN cargo build

# Run the binary
CMD ["./target/debug/core"]