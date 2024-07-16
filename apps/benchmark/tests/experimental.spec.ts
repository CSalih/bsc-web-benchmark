import { test } from "@playwright/test";

test.describe("experimental: chromium only", () => {
  test.skip(({ browserName }) => browserName !== "chromium", "Chromium only!");

  test("Get performance metrics", async ({ page }) => {
    // Create a new connection to an existing CDP session to enable performance Metrics
    const session = await page.context().newCDPSession(page);
    // Start record CDP session performance metrics
    await session.send("Performance.enable");
    await page.goto("/");
    const performanceMetrics = await session.send("Performance.getMetrics");

    console.log(performanceMetrics.metrics);
  });

  test("Capture performance traces by marking actions using Performance API", async ({
    page,
    browser,
  }) => {
    await browser.startTracing(page, {
      path: "./test-results/perf-profile.json",
      screenshots: true,
    });
    await page.goto("/");

    // Using Performance.mark API
    await page.evaluate(() =>
      window.performance.mark("home:heading:click_start"),
    );
    await page.getByRole("heading", { name: "Hello World" }).click();
    await page.evaluate(() =>
      window.performance.mark("home:heading:click_end"),
    );

    // To get all performance marks
    const getAllMarksJson = await page.evaluate(() =>
      JSON.stringify(window.performance.getEntriesByType("mark")),
    );
    console.log("Performance API:", JSON.parse(getAllMarksJson));

    // Performance measure
    await page.evaluate(() =>
      window.performance.measure(
        "home:heading:click_duration",
        "home:heading:click_start",
        "home:heading:click_end",
      ),
    );

    // To get all performance measures of Google
    const getAllMeasuresJson = await page.evaluate(() =>
      JSON.stringify(window.performance.getEntriesByType("measure")),
    );
    console.log("Performance API:", JSON.parse(getAllMeasuresJson));

    await browser.stopTracing();
  });
});
