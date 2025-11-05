# Build stage
FROM rust:alpine3.22 AS bulder
RUN apk add --no-cache musl-dev gcc libc-dev make
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo install --path .
COPY . .
RUN cargo build --release

# Runtime stage
FROM alpine:3.22
RUN apk add --no-cache ca-certificates tzdata && \
    adduser -D swiftcart
WORKDIR /app
COPY --from=bulder /app/target/release/swiftcart /usr/local/bin/swiftcart
USER swiftcart
EXPOSE 4000
ENTRYPOINT [ "swiftcart" ]
