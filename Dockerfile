FROM rust:1.64 AS builder
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder ./target/release/my-app ./my-app
COPY --from=builder ./.env ./.env
CMD ["my-app"]