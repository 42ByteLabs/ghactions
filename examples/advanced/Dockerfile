FROM docker.io/library/rust:1.78.0 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM docker.io/library/debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/ghactions /usr/local/bin/ghactions

CMD ["ghactions"]
