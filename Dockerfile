FROM rust:1.66 as builder
WORKDIR /usr/src/pt_register_check
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/pt_register_check /usr/local/bin/pt_register_check
CMD ["pt_register_check"]
