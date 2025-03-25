#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");
const process = require("process");
const { Command, Option } = require("commander");

const apps = {
  "app-angular": {
    baseUrl: "http://localhost:3000",
    testFile: "tests/responsiveness.spec.ts",
  },
  "app-leptos": {
    baseUrl: "http://localhost:3001",
    testFile: "tests/responsiveness.spec.ts",
  },
  "app-react": {
    baseUrl: "http://localhost:3002",
    testFile: "tests/responsiveness.spec.ts",
  },
  "app-vue": {
    baseUrl: "http://localhost:3003",
    testFile: "tests/responsiveness.spec.ts",
  },
  "realworld-angular": {
    baseUrl: "http://localhost:4000",
    testFile: "tests/initial-rendering.spec.ts",
  },
  "realworld-leptos": {
    baseUrl: "http://localhost:4001",
    testFile: "tests/initial-rendering.spec.ts",
  },
  "realworld-react": {
    baseUrl: "http://localhost:4002",
    testFile: "tests/initial-rendering.spec.ts",
  },
  "realworld-vue": {
    baseUrl: "http://localhost:4003",
    testFile: "tests/initial-rendering.spec.ts",
  },
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
  appName,
  baseUrl,
  workers = 1,
  repeatEach = 100,
  maxFailures = 100,
  retries = 5,
  testFile,
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
      testFile,
    ],
    {
      stdio: "inherit",
      env: {
        ...process.env,
        APP_NAME: appName,
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

  const isRunningCommand = spawnSync("docker", ["container", "inspect", "--format", "{{.State.Running}}", name], {
    stdio: "ignore",
  });
  const isRunning = isRunningCommand.status === 0;
  if (isRunning) {
    console.log(`${app} server is already running. Stopping...`);
    stopWebServer(name);
  }

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
      `csalih/${app}:latest`,
    ],
    {
      stdio: ['ignore', 'ignore', 'pipe'],
    },
  );

  if (cmd.status !== 0) {
    console.error(`Failed to start ${app} server! Exiting...`);
    console.error("#################");
    console.error(cmd.stderr?.toString() ?? "Unknown error! See docker logs for more information.");
    console.error("#################");
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
  .addOption(
    new Option("-a, --app [app...]").default("all").choices(Object.keys(apps)),
  )
  .addOption(new Option("-n, --repeat [repeat]").default(100))
  .addOption(new Option("-j, --workers [workers]").default(1))
  .action((options) => {
    const appsToRun = Object.keys(apps).filter((app) => {
      if (options.app === "all") {
        return true;
      }
      return options.app.includes(app);
    });

    console.log("Running benchmarks for", appsToRun.join(", "));

    appsToRun.forEach((appName) => {
      const app = apps[appName];
      const container = startWebServer(appName, app.baseUrl);
      runBenchmark({
        appName,
        baseUrl: app.baseUrl,
        workers: options.workers,
        repeatEach: options.repeat,
        testFile: app.testFile,
      });

      // backup results
      console.log(`Backing up results for ${appName}`);
      const os = require("os");
      const backupDir = path.join(
        __dirname,
        `../test-run/${appName}_${os.platform()}_${testDate}`,
      );
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
