FROM rust:1.83-slim AS build

WORKDIR /app

# Upgrade the system and install dependencies for PostgreSQL
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold \
  curl unzip build-essential pkg-config libssl-dev \
  postgresql-client libpq-dev

# Install diesel_cli for PostgreSQL
RUN cargo install diesel_cli --no-default-features --features "postgres"

COPY . .

RUN cargo build --release

COPY .env.example .env

EXPOSE 8080

RUN ls -la

# Set the startup command
CMD ["bash", "-c", "diesel migration run && /app/target/release/conduit"]
