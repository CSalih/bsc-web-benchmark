FROM rust:1.75.0-alpine AS build

WORKDIR /app

RUN apk add --no-cache musl-dev

RUN rustup update && \
    rustup target add wasm32-unknown-unknown

RUN wget https://github.com/trunk-rs/trunk/releases/download/v0.20.1/trunk-x86_64-unknown-linux-musl.tar.gz \
    && test "a6788406679e360bb5c70acd01eca8f33f6d2193732b7582e0dc424d115c8abe" = "$(sha256sum trunk-x86_64-unknown-linux-musl.tar.gz | awk '{print $1}')" \
    && tar -xvf trunk-x86_64-unknown-linux-musl.tar.gz \
    && mv trunk /usr/local/bin
RUN wget https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.92/wasm-bindgen-0.2.92-x86_64-unknown-linux-musl.tar.gz \
    && test "c6e43a3bf0be5231e0b72ea702f73b3f4f47c309037e8a332c5c2e41800ca934" = "$(sha256sum wasm-bindgen-0.2.92-x86_64-unknown-linux-musl.tar.gz | awk '{print $1}')" \
    && tar -xvf wasm-bindgen-0.2.92-x86_64-unknown-linux-musl.tar.gz \
    && mv wasm-bindgen-0.2.92-x86_64-unknown-linux-musl/wasm-bindgen /usr/local/bin

COPY Cargo.toml Cargo.lock ./
RUN cargo new apps/app-leptos
COPY apps/app-leptos/Cargo.toml apps/app-leptos/Cargo.toml
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo fetch --locked

COPY apps/app-leptos/ apps/app-leptos/
RUN --mount=type=cache,target=/usr/local/cargo/registry cd apps/app-leptos && trunk build --release --locked --offline

FROM nginx:1.26.0
COPY --from=build /app/apps/app-leptos/dist /usr/share/nginx/html

EXPOSE 80

STOPSIGNAL SIGTERM

ENTRYPOINT ["nginx", "-g", "daemon off;"]
