FROM rust:1.80-alpine3.20 as builder

WORKDIR /app

RUN apk add --no-cache musl-dev openssl-dev

ENV OPENSSL_DIR=/usr
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

COPY . .

RUN cargo build --release

FROM alpine:3.20

WORKDIR /app

COPY --from=builder /app/target/release/tdd_rs /app/tdd_rs

CMD ["/app/tdd_rs"]
EXPOSE 8065