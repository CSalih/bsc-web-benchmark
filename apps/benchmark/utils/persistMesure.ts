import * as fs from "node:fs";

const outputDir = "test-results/measure";

type PersistMeasureProps = {
  phase: string;
  measure: PerformanceMeasure;
};

export const persistMeasure = ({ phase, measure }: PersistMeasureProps) => {
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  fs.appendFileSync(
    `${outputDir}/${measure.name}-${phase}.txt`,
    `${measure.duration}\n`,
  );
};
