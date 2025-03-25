# Benchmark Analyze

This provides the Jupyter Notebook to analyze the benchmark results. 

## Requirements
- [uv](https://docs.astral.sh/uv/getting-started/installation/) v0.5 or later

After the benchmark, merge all `test-run/app-*/bechmark-results.csv`, from `benchmark-playwright`, results into one file under `data/app-results.csv`
and all `test-run/realworld-*/bechmark-results.csv` results into one file under `data/realworld-results.csv`.

## Start
```bash
pnpm jupyter
```

Select the project kernel from the dropdown menu.