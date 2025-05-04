# Use a build stage
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Use a smaller base image for final
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/PolitiWiki /app/PolitiWiki
EXPOSE 3000
CMD ["./PolitiWiki"]
