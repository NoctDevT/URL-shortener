FROM rust:1.84.1-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*  

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

COPY . .

RUN rm -rf /app/db /app/database

RUN cargo build --release --features dotenv

CMD ["./target/release/url-shortener"]
