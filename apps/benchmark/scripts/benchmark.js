#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");
const process = require("process");

const testDate = new Date().toISOString();

const apps = ["app-angular", "app-leptos", "app-react", "app-vue"];
const ignoreApps = ["app-angular", "app-react", "app-vue"];
const baseUrl = {
  "app-angular": "http://localhost:3000",
  "app-leptos": "http://localhost:3001",
  "app-react": "http://localhost:3002",
  "app-vue": "http://localhost:3003",
};

const dumpSystemInfo = (outputDir) => {
  const os = require("os");

  const systemInfo = {
    os: {
      type: os.type(),
      release: os.release(),
      platform: os.platform(),
      machine: os.machine(),
      version: os.version(),
    },
    cpu: {
      arch: os.arch(),
      cores: os.cpus(),
    },
    memory: {
      totalmem: `${(os.totalmem() / 1024 / 1024).toFixed(0)} MB`,
      freemem: `${(os.freemem() / 1024 / 1024).toFixed(0)} MB`,
    },
  };

  const outputPath = path.join(outputDir, "system-info.json");
  console.log("Dumping system info to", outputPath);
  fs.writeFileSync(outputPath, JSON.stringify(systemInfo, null, 2));
};

const backupResults = (outputDir) => {
  const measureDir = path.join(__dirname, "../test-results/measure");
  if (!fs.existsSync(measureDir)) {
    console.error(`Results directory '${measureDir}' not found! Exiting...`);
    process.exit(1);
  }
  const reportDir = path.join(__dirname, "../playwright-report");
  if (!fs.existsSync(reportDir)) {
    console.error(`Report directory '${reportDir}' not found! Exiting...`);
    process.exit(1);
  }
  if (!fs.existsSync(outputDir)) {
    console.error(`Output directory '${outputDir}' not found! Exiting...`);
    process.exit(1);
  }

  fs.cpSync(reportDir, `${outputDir}/report`, {
    recursive: true,
  });
  fs.cpSync(measureDir, outputDir, {
    recursive: true,
  });
};

const runBenchmark = (baseUrl) => {
  const benchmarkResult = spawnSync(
    "playwright",
    [
      "test",
      "--repeat-each=100",
      "--workers=4",
      "tests/responsiveness.spec.ts",
    ],
    {
      stdio: "inherit",
      env: {
        ...process.env,
        APP_BASE_URL: baseUrl,
      },
    },
  );

  if (benchmarkResult.status !== 0) {
    console.error(
      `Benchmark failed with status ${benchmarkResult.status}! Exiting...`,
    );
    process.exit(1);
  }
};

const startWebServer = (app) => {
  // docker run --rm -p 3000:80 bsc/app-angular:latest
  // docker run --rm -p 3001:80 bsc/app-leptos:latest
  // docker run --rm -p 3002:80 bsc/app-react:latest
  // docker run --rm -p 3003:80 bsc/app-vue:latest
};

apps
  .filter((app) => !ignoreApps.includes(app))
  .forEach((app) => {
    const url = baseUrl[app];
    runBenchmark(url);

    // backup results
    console.log(`Backing up results for ${app}`);
    const backupDir = path.join(__dirname, `../test-run/${app}-${testDate}`);
    if (!fs.existsSync(backupDir)) {
      fs.mkdirSync(backupDir, {
        recursive: true,
      });
    }
    backupResults(backupDir);
    dumpSystemInfo(backupDir);
  });
