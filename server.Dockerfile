FROM rust:1.70 as builder

WORKDIR /app

RUN update-ca-certificates

COPY server/dummy.rs ./dummy.rs
COPY server/Cargo.toml ./Cargo.toml

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY server/src ./src

RUN cargo build --release

FROM ubuntu:20.04

RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/server ./

CMD ["./server"]
