FROM rust:1.70-alpine AS builder

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder ./target/release/rust-axum-seaorm-postgres ./target/release/rust-axum-seaorm-postgres

CMD ["/target/release/rust-axum-seaorm-postgres"]