FROM rust:1.64.0-buster as build

WORKDIR /app/

RUN apt-get update && apt-get install -y cmake libclang-dev libc++-dev gcc-multilib && apt-get clean

COPY feature-pipe/ /app/feature-pipe/
COPY rust-transformer/ /app/rust-transformer/
COPY src/ /app/src/
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock

RUN cargo build --release


FROM debian:latest

WORKDIR /app/

RUN apt-get update && apt-get install -y cmake libclang-dev libc++-dev gcc-multilib && apt-get clean

COPY --from=build /app/target/release/fast-lgbm-inference .

EXPOSE 8080

CMD ["/app/fast-lgbm-inference"]