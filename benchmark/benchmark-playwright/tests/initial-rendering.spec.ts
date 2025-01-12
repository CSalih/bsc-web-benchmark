import { resolve } from "path";
import { expect, test } from "@playwright/test"
import { baseUrl } from "../utils/base-url";

test.describe("initial rendering phase", () => {
  test("web_vitals", async ({ page }) => {
    // Ignore analytics when warming up
    await page.route("http://localhost:8000/api/v1/event", (route) => {
      route.fulfill({ status: 204 });
    });

    await page.addInitScript({
      // TODO: may parse script and replace project name, environment and version
      path: resolve(__dirname, "../utils/web-vitals.js"),
    });

    await page.goto(baseUrl("/"), {
      waitUntil: "networkidle",
    });

    // Validate articles are loaded and rendered to the DOM
    await expect(page.getByRole("heading", {
      name: "Playwright"
    })).toBeAttached();

    // click on an element to get Web Vitals like INP
    await page.getByRole("heading", {
      name: "conduit"
    }).click();
  });
});
