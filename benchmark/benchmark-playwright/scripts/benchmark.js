#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");
const process = require("process");
const { Command, Option } = require("commander");

const apps = ["app-angular", "app-leptos", "app-react", "app-vue"];
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

const runBenchmark = ({
  baseUrl,
  workers = 1,
  repeatEach = 100,
  maxFailures = 10,
  retries = 3,
}) => {
  console.log("Starting benchmark");
  const benchmarkResult = spawnSync(
    "npx",
    [
      "playwright",
      "test",
      `--repeat-each=${repeatEach}`,
      `--workers=${workers}`,
      `--max-failures=${maxFailures}`,
      `--retries=${retries}`,
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
  }
};

const startWebServer = (app, baseUrl) => {
  const name = `benchmark-${app}`;
  const url = new URL(baseUrl);

  console.log(`Starting ${app} server on port ${url.port} ...`);
  const cmd = spawnSync(
    "docker",
    [
      "run",
      `--detach`,
      "-p",
      `${url.port}:80`,
      "--name",
      name,
      `bsc/${app}:latest`,
    ],
    {
      stdio: "ignore",
    },
  );

  if (cmd.status !== 0) {
    console.error(`Failed to start ${app} server! Exiting...`);
    console.error(cmd.stderr.toString());
    process.exit(1);
  }
  return name;
};

const stopWebServer = (name) => {
  console.log(`Stopping web server for ${name} ...`);
  spawnSync("docker", ["stop", name], {
    stdio: "ignore",
  });
  spawnSync("docker", ["rm", name], {
    stdio: "ignore",
  });
};

const testDate = new Date().valueOf();
const program = new Command();
program
  .name("benchmark")
  .description("CLI to some JavaScript string utilities")
  .version("0.1.0");

program
  .command("run", {
    isDefault: true,
  })
  .description("Run the benchmark")
  .addOption(new Option("-a, --app [app...]").default("all").choices(apps))
  .addOption(new Option("-n, --repeat [repeat]").default(100))
  .addOption(new Option("-j, --workers [workers]").default(1))
  .action((options) => {
    const appsToRun = apps.filter((app) => {
      if (options.app === "all") {
        return true;
      }
      return options.app.includes(app);
    });

    console.log("Running benchmarks for", appsToRun.join(", "));

    appsToRun.forEach((app) => {
      const url = baseUrl[app];
      const container = startWebServer(app, url);
      runBenchmark({
        baseUrl: url,
        workers: options.workers,
        repeatEach: options.repeat,
        maxFailures: 50
      });

      // backup results
      console.log(`Backing up results for ${app}`);
      const os = require("os");
      const backupDir = path.join(__dirname, `../test-run/${app.replaceAll("-", "_")}-${os.platform()}-${testDate}`);
      if (!fs.existsSync(backupDir)) {
        fs.mkdirSync(backupDir, {
          recursive: true,
        });
      }
      backupResults(backupDir);
      dumpSystemInfo(backupDir);

      stopWebServer(container);
    });
  });

program.parse();
