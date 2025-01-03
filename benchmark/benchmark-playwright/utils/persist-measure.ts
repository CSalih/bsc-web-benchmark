import * as fs from "node:fs";
import { type Page, test } from "@playwright/test";

const outputDir = "test-results/measure";

export const persistMeasure = (page: Page, measure: PerformanceMeasure) => {
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  const browser = page.context().browser().browserType().name();

  fs.appendFileSync(
    `${outputDir}/${measure.name}-${browser}.txt`,
    `${measure.duration}\n`,
  );
  test.info().annotations.push({
    type: "measure",
    description: `${measure.name}: ${measure.duration.toFixed(2)}ms`,
  });
};
