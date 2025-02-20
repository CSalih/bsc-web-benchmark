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

```bash
cd benchmark/benchmark-playwright/
node scripts/benchmark.js
```
