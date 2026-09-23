FROM rust:1.98-alpine3.24 AS chef
RUN apk add --no-cache musl-dev g++ make cmake
RUN cargo install cargo-chef

FROM chef AS planner
WORKDIR /build
COPY ./crates ./crates
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS backend-builder
WORKDIR /build
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./crates ./crates
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

FROM alpine:3.24 AS runtime
WORKDIR /app
COPY --from=backend-builder /build/target/release/api .
CMD ["./api"]