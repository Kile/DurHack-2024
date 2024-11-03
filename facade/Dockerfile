# Use the official Rust image to build the project
FROM rust:1.82 AS builder

WORKDIR /app
COPY . .

# Build the project
RUN cargo build

# Expose the port the API runs on
EXPOSE 8080

# Run the binary
CMD ["./target/debug/facade"]