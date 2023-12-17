FROM rust:latest as builder
WORKDIR /usr/src/a-t_3
COPY . .
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y musl-tools libssl-dev && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl
FROM scratch
WORKDIR /usr/src/a-t_3
COPY --from=builder /usr/src/a-t_3/target/x86_64-unknown-linux-musl/release/a-t_3 /usr/src/a-t_3/
CMD ["./a-t_3"]
