FROM rust:latest

WORKDIR /app

# Install cargo-watch for hot reloading
RUN cargo install cargo-watch

# Copy everything into the container
COPY . .

EXPOSE 3000

CMD ["cargo", "run","--release"]
