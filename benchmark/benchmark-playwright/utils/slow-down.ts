import { type Page } from "@playwright/test";
import { test } from "@playwright/test";

export const withCPUSlowdown = async (
  page: Page,
  rate: number,
  fn: () => Promise<void>,
) => {
  const cdp = await page.context().newCDPSession(page);
  await cdp.send("Emulation.setCPUThrottlingRate", {
    rate,
  });
  test.info().annotations.push({
    type: "emulation",
    description: `CPUThrottlingRate set to ${rate}x slowdown`,
  });

  await fn();

  // Reset CPU throttling
  await cdp.send("Emulation.setCPUThrottlingRate", {
    rate: 1,
  });
};
