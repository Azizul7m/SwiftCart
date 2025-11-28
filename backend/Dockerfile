FROM rust:alpine3.22 AS builder
RUN apk add --no-cache musl-dev gcc libc-dev make
WORKDIR /app
COPY . .
RUN cargo install --path .
RUN cargo build --release

FROM alpine:3.22
RUN apk add --no-cache ca-certificates && \
    adduser -D swiftcart
WORKDIR /app
COPY --chown=swiftcart:swiftcart --from=builder /app/target/release/swiftcart /usr/local/bin
USER swiftcart
EXPOSE 4000
ENTRYPOINT [ "swiftcart" ]

