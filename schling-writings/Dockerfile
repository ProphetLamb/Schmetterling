FROM rust:latest

ENV CARGO_TERM_COLOR always
RUN apt-get update && apt-get install -y libpq-dev cmake

# create empty project for caching dependencies
RUN USER=root cargo new --bin /schling-writings/docker-build
WORKDIR /schling-writings/docker-build
COPY /Cargo.lock ./
COPY /schling-writings/Cargo.toml ./
# cache dependencies
RUN cargo install --path . --locked

COPY /schling-writings/ ./
RUN cargo install --path . --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=0 /usr/local/cargo/bin/schling-writings /usr/local/bin/schling-writings
CMD ["schling-writings"]
EXPOSE 8001