import { resolve } from "path";
import { test } from "@playwright/test";
import { baseUrl } from "../utils/base-url";

test.describe("initial rendering phase", () => {
  test("warm up phase", async ({ page }) => {
    // Ignore analytics when warming up
    await page.route("http://localhost:8000/api/v1/event", (route) => {
      route.fulfill({ status: 204 });
    });

    await page.addInitScript({
      // TODO: parse script and replace project name, environment and version
      path: resolve(__dirname, "../utils/web-vitals.js"),
    });

    // open page
    await page.goto(baseUrl("/"), {
      timeout: 1000, // 1 second
      waitUntil: "networkidle",
    });

    // TODO: Validate elements are available in the DOM
    // await page.waitForSelector('h1')

    // click on an element to get Web Vitals like INP
    await page.getByRole("heading").click();
  });

  test("test phase", async ({ page }) => {
    const n = 100;

    for (let i = 0; i < n; i++) {
      await page.goto(baseUrl("/"));
      await page.getByRole("heading").click();
    }
  });
});
