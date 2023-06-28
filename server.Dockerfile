FROM rust:1.70 as builder

WORKDIR /app

RUN update-ca-certificates

ENV USER=app
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY server/Cargo.toml ./Cargo.toml
COPY server/src ./src

RUN cargo build --release --features docker

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

COPY --from=builder /app/target/release/server ./

USER app:app

CMD ["./server"]
