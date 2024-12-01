FROM rust:1.82.0-bullseye as builder
ENV APP atBB
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
 
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
ENTRYPOINT [ "/usr/local/bin/atBB" ]