FROM rustlang/rust:nightly-bullseye as builder

RUN cargo install --locked cargo-leptos

WORKDIR /app

#needed to allow correct installation of target
COPY rust-toolchain.toml .

RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rust-src

COPY . .

ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu"

RUN cargo update 
RUN cargo leptos --manifest-path=./Cargo.toml build --release -vv


FROM rustlang/rust:nightly-bullseye as runner

ARG LAST_UPDATED
ENV LAST_UPDATED=${LAST_UPDATED}

COPY --from=builder /app/posts /app/posts
COPY --from=builder /app/public /app/public

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/itehax-website /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="itehax-website"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
CMD [ "/app/itehax-website" ]