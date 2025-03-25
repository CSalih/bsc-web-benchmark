# Benchmark Playwright

This provides the test suit to benchmark the web frameworks using Playwright.

## Requirements

- [Node](https://nodejs.org/en/download) v20

## Setup

```bash
# Install the Playwright dependencies
pnpm exec playwright install
```

## Benchmark

Benchmark the responsiveness of the web frameworks using Playwright.

```bash
# Angular
pnpm run benchmark --app app-angular --repeat 100

# Leptos
pnpm run benchmark --app app-leptos --repeat 100

# React
pnpm run benchmark --app app-react --repeat 100

# Vue
pnpm run benchmark --app app-vue --repeat 100
```

Benchmark the initial rendering phase of the web frameworks using Playwright.

```bash
# Start the RealWorld API server
pnpm start:api

# Angular
pnpm run benchmark --app realworld-angular --repeat 1000

# Leptos
pnpm run benchmark --app realworld-leptos --repeat 1000

# React
pnpm run benchmark --app realworld-react --repeat 1000

# Vue
pnpm run benchmark --app realworld-vue --repeat 1000
```

## Results

After each benchmark, the results with the metadata are stored in the `test-run` folder.
To analyze the results, use the [Benchmark Analyze](../benchmark-analyze/README.md) Jupyter Notebook.