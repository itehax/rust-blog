FROM rustlang/rust:nightly-bullseye as builder

WORKDIR /app

COPY rust-toolchain.toml .

RUN rustup show \
    && rustup target add wasm32-unknown-unknown \
    && rustup component add rust-src


RUN cargo install cargo-leptos --version 0.2.21 --locked

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true && rm -rf src

COPY . .
ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu"

RUN cargo leptos --manifest-path=./Cargo.toml build --release -vv

# --- RUNNER STAGE ---
FROM debian:bullseye-slim as runner


ARG LAST_UPDATED
ENV LAST_UPDATED=$LAST_UPDATED

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/itehax-website /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/posts /app/posts
COPY --from=builder /app/public /app/public

WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="itehax-website"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000

CMD [ "/app/itehax-website" ]