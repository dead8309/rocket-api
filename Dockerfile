FROM rust:1.75.0-buster as builder

WORKDIR /app

ARG MONGOURI

ENV MONGOURI=$MONGOURI

COPY . . 

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rocket-api .
COPY --from=builder /app/Rocket.toml .

CMD ["./rocket-api"]