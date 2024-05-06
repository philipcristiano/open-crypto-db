FROM rust:1.77-bookworm as builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock /usr/src/app/
RUN \
    mkdir /usr/src/app/src && \
    echo 'fn main() {}' > /usr/src/app/src/main.rs && \
    cargo build --release && \
    rm -Rvf /usr/src/app/src

COPY . .
RUN touch src/main.rs
RUN cargo build --release -v
RUN cargo run --

FROM nginx:stable-alpine3.19-slim
COPY --from=builder /usr/src/app/output/ /usr/share/nginx/html
