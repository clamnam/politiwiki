# Use a build stage
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Use a smaller base image for final
FROM debian:bullseye-slim
WORKDIR /app

# Install SSL dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/politiwiki /app/politiwiki
EXPOSE 3000
CMD ["./politiwiki"]
