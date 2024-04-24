# Use a Rust base image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy your Rust project files
COPY . .

# Install dependencies using Cargo
RUN cargo fetch

# Build your Rust project
RUN cargo build --release

# Final stage: minimal runtime image
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/release/threadserver /app/server

# Install additional dependencies
RUN apt-get update && \
    apt-get install -y \
    openssl \
    && rm -rf /var/lib/apt/lists/*

# Expose the port your Rust server listens on
EXPOSE 8080

# Set the entrypoint for your Rust server binary
CMD ["/app/server", "tokio", "4"]
