FROM rust:1.75.0-alpine AS build

WORKDIR /build

RUN apk add --no-cache musl-dev

RUN rustup update && \
    rustup target add wasm32-unknown-unknown

RUN wget https://github.com/trunk-rs/trunk/releases/download/v0.20.1/trunk-x86_64-unknown-linux-musl.tar.gz \
    && test "a6788406679e360bb5c70acd01eca8f33f6d2193732b7582e0dc424d115c8abe" = "$(sha256sum trunk-x86_64-unknown-linux-musl.tar.gz | awk '{print $1}')" \
    && tar -xvf trunk-x86_64-unknown-linux-musl.tar.gz \
    && mv trunk /usr/local/bin

COPY . .

RUN trunk build --release --locked

FROM nginx:1.26.0
COPY --from=build /build/dist /usr/share/nginx/html

EXPOSE 80

STOPSIGNAL SIGTERM

CMD ["nginx", "-g", "daemon off;"]
