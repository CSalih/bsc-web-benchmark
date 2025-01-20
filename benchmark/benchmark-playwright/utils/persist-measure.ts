import * as fs from "node:fs";
import { type Page, test } from "@playwright/test";

const outputDir = "test-results/measure";

type WebVitalMetric = {
  /**
   * The name of the metric (in acronym form).
   */
  name: 'CLS' | 'FCP' | 'FID' | 'INP' | 'LCP' | 'TTFB';
  /**
   * The current value of the metric.
   */
  value: number;
  /**
   * The rating as to whether the metric value is within the "good",
   * "needs improvement", or "poor" thresholds of the metric.
   */
  rating: 'good' | 'needs-improvement' | 'poor';
  /**
   * Any performance entries relevant to the metric value calculation.
   * The array may also be empty if the metric value was not based on any
   * entries (e.g. a CLS value of 0 given no layout shifts).
   */
  entries: PerformanceEntry[];
}


export const persistWebVital = (page: Page, result: WebVitalMetric[]) => {
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  const filename = `${outputDir}/benchmark-web-vitals.csv`;
  const browser = page.context().browser().browserType().name();
  const operatingSystem = process.platform;
  const appName = process.env.APP_NAME || "unknown";

  for (const entry of result) {
    const row = [
      appName,
      operatingSystem,
      browser,
      entry.name,
      entry.rating,
      entry.value
    ].join(",");

    fs.appendFileSync(filename, `${row}\n`);
    test.info().annotations.push({
      type: "measure",
      description: `${entry.name}: ${entry.value.toFixed(2)}ms`,
    });
  }
};

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
