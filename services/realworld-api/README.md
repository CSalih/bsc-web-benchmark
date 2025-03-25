# RealWorld API

Realworld App using `Rust`, `actix-web`, and `diesel`.
Forked from [snamiki1212/realworld-v1-rust-actix-web-diesel](https://github.com/snamiki1212/realworld-v1-rust-actix-web-diesel).

## Getting Started

```zsh
docker compose up -d

# healthcheck
curl http://localhost:8080/api/healthcheck
```

## E2E Test

Running E2E tests using [POSTMAN scripts](https://github.com/gothinkster/realworld/tree/main/api) on CI

```zsh
# run e2e
$ APIURL=http://localhost:8080/api zsh e2e/run-api-tests.sh
```
## LICENSE

MIT
