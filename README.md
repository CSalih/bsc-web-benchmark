# Web Benchmark

Benchmark comparison of Leptos to Leading JavaScript Web Frameworks

# Requirements

- Node v20
- Docker v23 or later

# Setup

```bash
# Install pnpm
corepack enable

# Install dependencies
pnpm install

# Build the Docker image
pnpm build:docker

# Install the Playwright dependencies
pnpm --filter=benchmark-playwright exec playwright install
```

# Benchmark

To execute the benchmark use the following command:

```bash
pnpm run benchmark
```

The benchmark script accepts the following arguments:

* `--app` Space-separated list of frameworks to benchmark. Default:
  `app-leptos app-react app-angular app-vue realworld-leptos realworld-react realworld-angular realworld-vue`
* `--repeat` Number of iterations to run the benchmark. Default: `100`

> *NOTE:*
> For the `realworld-*` benchmarks you need to start the RealWorld API server first.
> You can do this by running the following command: `pnpm start:api`

Example to benchmark Leptos with 100 iterations:

```bash
pnpm start:api
pnpm run benchmark --app realworld-leptos --repeat 100
```
