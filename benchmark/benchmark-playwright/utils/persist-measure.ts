import * as fs from "node:fs";
import { type Page, test } from "@playwright/test";

const outputDir = "test-results/measure";

export const persistMeasure = (page: Page, measure: PerformanceMeasure) => {
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  const filename = `${outputDir}/benchmark-results.csv`;
  const browser = page.context().browser().browserType().name();
  const operatingSystem = process.platform;
  const appName = process.env.APP_NAME || "unknown";

  const row = [
    appName,
    measure.name,
    operatingSystem,
    browser,
    measure.duration,
  ].join(",");

  fs.appendFileSync(filename, `${row}\n`);
  test.info().annotations.push({
    type: "measure",
    description: `${measure.name}: ${measure.duration.toFixed(2)}ms`,
  });
};
