import { test } from "@playwright/test";
import { afterFrame } from "../utils/after-frame";
import { baseUrl } from "../utils/baseUrl";
import { persistMeasure } from "../utils/persistMesure";

test("create_1000_rows", async ({ page }) => {
  // Add custom script to measure performance
  await page.addInitScript(afterFrame);

  // Emulate slow CPU
  const client = await page.context().newCDPSession(page);
  await client.send("Emulation.setCPUThrottlingRate", {
    rate: 3,
  });

  // open page
  await page.goto(baseUrl("/"), {
    timeout: 1000, // 1 second
    waitUntil: "networkidle",
  });

  // Validate elements are available in the DOM
  await test.expect(page.locator("#run")).toBeVisible();
  await test.expect(page.locator(".table > tbody")).toBeEmpty();

  // Create 1000 rows
  await page.evaluate(() => {
    performance.mark("create_1000_start");
    document.getElementById("run").click();
    // @ts-ignore: We are sure that the function is available in the window object
    window.afterFrame(() => {
      performance.mark("create_1000_end");
    });
  });

  // wait for table to render and validate the number of rows
  await page
    .locator(
      ".table > tbody:nth-child(1) > tr:nth-child(1000) > td:nth-child(1)",
    )
    .waitFor({
      state: "attached",
      timeout: 1000,
    });
  await test.expect(page.locator(".table > tbody > tr")).toHaveCount(1000);

  // Calculate the duration of the action
  const measure = await page.evaluate(() => {
    return performance.measure(
      "create_1000_duration",
      "create_1000_start",
      "create_1000_end",
    );
  });
  persistMeasure({ measure, phase: "warmup" });
});
