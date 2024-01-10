FROM rust:1.64 AS builder
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN groupadd --gid 1000 rust \
  && useradd --uid 1000 --gid rust --shell /bin/bash --create-home rust
USER rust
WORKDIR /home/rust/app
COPY --from=builder --chown=rust ./target/release/my-app ./my-app
COPY --from=builder --chown=rust ./.env ./.env
CMD ["./my-app"]