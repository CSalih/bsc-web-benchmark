import * as fs from "node:fs";
import {test} from "@playwright/test";

const outputDir = "test-results/measure";

type PersistMeasureProps = {
  measure: PerformanceMeasure;
};

export const persistMeasure = ({ measure }: PersistMeasureProps) => {
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  fs.appendFileSync(
    `${outputDir}/${measure.name}.txt`,
    `${measure.duration}\n`,
  );
  test.info().annotations.push(({
    type: 'measure',
    description: `${measure.name}: ${measure.duration.toFixed(2)}ms`
  }))
};
