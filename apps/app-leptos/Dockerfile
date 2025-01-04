FROM rust:1.83.0-alpine AS build

WORKDIR /app

RUN apk add --no-cache musl-dev

RUN rustup update && \
    rustup target add wasm32-unknown-unknown

RUN wget https://github.com/trunk-rs/trunk/releases/download/v0.21.5/trunk-x86_64-unknown-linux-musl.tar.gz \
    && test "82be12cb94f43494ab17014c6f69c407005f2c583d7a4b369ecb990be6168a8d" = "$(sha256sum trunk-x86_64-unknown-linux-musl.tar.gz | awk '{print $1}')" \
    && tar -xvf trunk-x86_64-unknown-linux-musl.tar.gz \
    && mv trunk /usr/local/bin
RUN wget https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.99/wasm-bindgen-0.2.99-x86_64-unknown-linux-musl.tar.gz \
    && test "0ae33d77b6f4fe09b294fa245764dbb05a3a1aeefeb7696cb365760d3abbdc51" = "$(sha256sum wasm-bindgen-0.2.99-x86_64-unknown-linux-musl.tar.gz | awk '{print $1}')" \
    && tar -xvf wasm-bindgen-0.2.99-x86_64-unknown-linux-musl.tar.gz \
    && mv wasm-bindgen-0.2.99-x86_64-unknown-linux-musl/wasm-bindgen /usr/local/bin

COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo fetch --locked

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry trunk build --release --locked --offline

FROM nginx:1.27.3-alpine
COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 80

STOPSIGNAL SIGTERM

ENTRYPOINT ["nginx", "-g", "daemon off;"]
