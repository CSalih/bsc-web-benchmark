# BSc Web Benchmark

Benchmark comparison of Leptos to Leading JavaScript Web Frameworks

# Requirements

- Node v20
- Docker v23 or later

# Setup

```bash
corepack enable
pnpm install
pnpm build:docker
pnpm --filter=benchmark-playwright exec playwright install
```

# Benchmark

To execute the benchmark, run the following command:

The benchmark script accepts the following arguments:

* `--app` - Space-separated list of frameworks to benchmark. Default:
  `app-leptos,app-react,app-angular,app-vue,realworld-leptos,realworld-react,realworld-angular,realworld-vue`
* `--repeat` - Number of iterations to run the benchmark. Default: `100`

> *NOTE:*
> For the `realworld-*` benchmarks you need to start the RealWorld API server first.
> You can do this by running the following command:
> `cd services/realworld-api && docker compose up -d`

```bash
cd benchmark/benchmark-playwright/
# Benchmark Leptos with 100 iterations
node scripts/benchmark.js --app app-leptos --repeat 100
```
