import { test } from "@playwright/test"
import { afterFrame } from "../utils/after-frame"
import { baseUrl } from "../utils/base-url"
import { persistMeasure } from "../utils/persist-measure"

test.describe.configure({ mode: "parallel" })

test("create_1000_rows", async ({ page }) => {
  // Add custom script to measure performance
  await page.addInitScript(afterFrame)

  // open page
  await page.goto(baseUrl("/"))

  // Validate elements are available in the DOM
  await test.expect(page.locator("#run")).toBeVisible()
  await test.expect(page.locator(".table > tbody")).toBeEmpty()

  // Create 1000 rows
  await page.evaluate(() => {
    performance.mark("create_1000_start")
    document.getElementById("run").click()
    // @ts-ignore: We are sure that the function is available in the window object
    window.afterFrame(() => {
      performance.mark("create_1000_end")
    })
  })

  // wait for table to render and validate the number of rows
  await page
    .locator(
      ".table > tbody:nth-child(1) > tr:nth-child(1000) > td:nth-child(1)",
    )
    .waitFor({
      state: "attached",
    })
  await test.expect(page.locator(".table > tbody > tr")).toHaveCount(1000)

  // Wait for a second so metrics are available
  await page.waitForTimeout(1000)

  // Calculate the duration of the action
  const measure = await page.evaluate(() => {
    return performance.measure(
      "create_1000_duration",
      "create_1000_start",
      "create_1000_end",
    )
  })
  persistMeasure(page, measure)
})

test("create_10000_rows", async ({ page }) => {
  // Add custom script to measure performance
  await page.addInitScript(afterFrame)

  // open page
  await page.goto(baseUrl("/"))

  // Validate elements are available in the DOM
  await test.expect(page.locator("#runlots")).toBeVisible()
  await test.expect(page.locator(".table > tbody")).toBeEmpty()

  // Create 1000 rows
  await page.evaluate(() => {
    performance.mark("create_10000_start")
    document.getElementById("runlots").click()
    // @ts-ignore: We are sure that the function is available in the window object
    window.afterFrame(() => {
      performance.mark("create_10000_end")
    })
  })

  // wait for table to render and validate the number of rows
  await page
    .locator(
      ".table > tbody:nth-child(1) > tr:nth-child(10000) > td:nth-child(1)",
    )
    .waitFor({
      timeout: 10000,
      state: "attached",
    })
  await test.expect(page.locator(".table > tbody > tr")).toHaveCount(10000, {
    timeout: 10000,
  })

  // Wait for a second so metrics are available
  await page.waitForTimeout(1000)

  // Calculate the duration of the action
  const measure = await page.evaluate(() => {
    return performance.measure(
      "create_10000_duration",
      "create_10000_start",
      "create_10000_end",
    )
  })
  persistMeasure(page, measure)
})

test("append_1000_rows", async ({ page }) => {
  // Add custom script to measure performance
  await page.addInitScript(afterFrame)

  // open page
  await page.goto(baseUrl("/"))

  // Validate elements are available in the DOM
  await test.expect(page.locator("#add")).toBeVisible()
  await test.expect(page.locator("#run")).toBeVisible()
  await test.expect(page.locator(".table > tbody")).toBeEmpty()

  // Create 1000 rows
  await page.evaluate(() => {
    document.getElementById("run").click()
  })
  // wait for table to render and validate the number of rows
  await page
    .locator(
      ".table > tbody:nth-child(1) > tr:nth-child(1000) > td:nth-child(1)",
    )
    .waitFor({
      state: "attached",
    })

  await page.evaluate(() => {
    performance.mark("append_1000_rows_start")
    document.getElementById("add").click()
    // @ts-ignore: We are sure that the function is available in the window object
    window.afterFrame(() => {
      performance.mark("append_1000_rows_end")
    })
  })

  // wait for table to render and validate the number of rows
  await page
    .locator(
      ".table > tbody:nth-child(1) > tr:nth-child(2000) > td:nth-child(1)",
    )
    .waitFor({
      state: "attached",
    })
  await test.expect(page.locator(".table > tbody > tr")).toHaveCount(2000)

  // Wait for a second so metrics are available
  await page.waitForTimeout(1000)

  // Calculate the duration of the action
  const measure = await page.evaluate(() => {
    return performance.measure(
      "append_1000_rows_duration",
      "append_1000_rows_start",
      "append_1000_rows_end",
    )
  })
  persistMeasure(page, measure)
})

test("update_every_10th_row", async ({ page }) => {
  // Add custom script to measure performance
  await page.addInitScript(afterFrame)

  // open page
  await page.goto(baseUrl("/"))

  // Validate elements are available in the DOM
  await test.expect(page.locator("#run")).toBeVisible()
  await test.expect(page.locator("#update")).toBeVisible()
  await test.expect(page.locator(".table > tbody")).toBeEmpty()

  // Create 10000 rows
  await page.evaluate(() => {
    document.getElementById("run").click()
  })
  // wait for table to render and validate the number of rows
  await page
    .locator(
      ".table > tbody:nth-child(1) > tr:nth-child(1000) > td:nth-child(1)",
    )
    .waitFor({
      state: "attached",
    })
  await test.expect(page.locator(".table > tbody > tr")).toHaveCount(1000)

  await page.evaluate(() => {
    performance.mark("update_every_10th_row_start")
    document.getElementById("update").click()
    // @ts-ignore: We are sure that the function is available in the window object
    window.afterFrame(() => {
      performance.mark("update_every_10th_row_end")
    })
  })

  // check if some 10th row is updated
  await test
    .expect(
      page.locator(
        ".table > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(2)",
      ),
    )
    .toContainText("!!!")
  await test
    .expect(
      page.locator(
        ".table > tbody:nth-child(1) > tr:nth-child(11) > td:nth-child(2)",
      ),
    )
    .toContainText("!!!")
  await test
    .expect(
      page.locator(
        ".table > tbody:nth-child(1) > tr:nth-child(981) > td:nth-child(2)",
      ),
    )
    .toContainText("!!!")
  await test
    .expect(
      page.locator(
        ".table > tbody:nth-child(1) > tr:nth-child(991) > td:nth-child(2)",
      ),
    )
    .toContainText("!!!")

  // Wait for a second so metrics are available
  await page.waitForTimeout(1000)

  // Calculate the duration of the action
  const measure = await page.evaluate(() => {
    return performance.measure(
      "update_every_10th_row_duration",
      "update_every_10th_row_start",
      "update_every_10th_row_end",
    )
  })
  persistMeasure(page, measure)
})

test("swap_rows", async ({ page }) => {
  // Add custom script to measure performance
  await page.addInitScript(afterFrame)

  // open page
  await page.goto(baseUrl("/"))

  // Validate elements are available in the DOM
  await test.expect(page.locator("#run")).toBeVisible()
  await test.expect(page.locator("#swaprows")).toBeVisible()
  await test.expect(page.locator(".table > tbody")).toBeEmpty()

  // Create 10000 rows
  await page.evaluate(() => {
    document.getElementById("run").click()
  })
  // wait for table to render and validate the number of rows
  await page
    .locator(
      ".table > tbody:nth-child(1) > tr:nth-child(1000) > td:nth-child(1)",
    )
    .waitFor({
      state: "attached",
    })
  await test.expect(page.locator(".table > tbody > tr")).toHaveCount(1000)
  await test
    .expect(
      page.locator(
        ".table > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(1)",
      ),
    )
    .toContainText("2")
  await test
    .expect(
      page.locator(
        ".table > tbody:nth-child(1) > tr:nth-child(999) > td:nth-child(1)",
      ),
    )
    .toContainText("999")

  // Wait for a second so metrics are available
  await page.waitForTimeout(1000)

  await page.evaluate(() => {
    performance.mark("swap_rows_start")
    document.getElementById("swaprows").click()
    // @ts-ignore: We are sure that the function is available in the window object
    window.afterFrame(() => {
      performance.mark("swap_rows_end")
    })
  })

  // check if 2nd and 999th rows are swapped
  await test
    .expect(
      page.locator(
        ".table > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(1)",
      ),
    )
    .toContainText("999")
  await test
    .expect(
      page.locator(
        ".table > tbody:nth-child(1) > tr:nth-child(999) > td:nth-child(1)",
      ),
    )
    .toContainText("2")

  // Calculate the duration of the action
  const measure = await page.evaluate(() => {
    return performance.measure(
      "swap_rows_duration",
      "swap_rows_start",
      "swap_rows_end",
    )
  })
  persistMeasure(page, measure)
})
