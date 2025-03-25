# Web Framework Benchmark

This repository contains the source code of the bachelor's thesis 
*Benchmark comparison of Leptos to Leading JavaScript Web Frameworks*.

The `apps` folder contains the source code of the used web frameworks, 
while the `benchmark` folder contains the benchmark related code.
The `services` folder contains the source code of the RealWorld API server.

> *NOTE:*
> In the thesis, the term RealWorld is referred to as Conduit. The code was
> not adapted to reflect this change yet.

# Requirements

- [Node](https://nodejs.org/en/download) v20
- [Docker](https://docs.docker.com/engine/install/) v23 or later
- [uv](https://docs.astral.sh/uv/getting-started/installation/) v0.5 or later

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
