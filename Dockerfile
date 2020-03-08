FROM rust:1.41-buster as builder
WORKDIR /usr/src/NATS-WebUI
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates libssl-dev
COPY --from=builder /usr/src/NATS-WebUI/target/release /usr/local/bin/NATS-WebUI
CMD ["/usr/local/bin/NATS-WebUI/NATS-WebUI"]