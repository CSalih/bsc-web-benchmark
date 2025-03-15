import { resolve } from "path"
import { expect, test } from "@playwright/test"
import { baseUrl } from "../utils/base-url"
import { persistWebVital } from "../utils/persist-measure"

test.describe("initial rendering phase", () => {
  test("web_vitals", async ({ page }) => {
    await page.addInitScript({
      path: resolve(__dirname, "../utils/web-vitals.js"),
    })

    await page.goto(baseUrl("/"))

    // Validate articles are loaded and rendered to the DOM
    await expect(
      page.getByRole("heading", {
        name: "Playwright",
      }),
    ).toBeAttached()

    // Click on a link to get Web Vitals like INP
    await page
      .getByRole("heading", {
        name: "Playwright",
      })
      .click()
    await expect(
      page.getByRole("heading", {
        name: "Summary of Playwright",
      }),
    ).toBeVisible()

    // Wait for a second so metrics are available
    await page.waitForTimeout(1000)

    // Persist the Web Vitals
    const results = await page.evaluate(() => {
      // @ts-ignore: webVitals is added by the injected script
      return window.webVitals
    })
    persistWebVital(page, results)
  })
})
