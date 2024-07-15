import { resolve } from 'path';
import {test} from '@playwright/test';
import {afterFrame} from "../utils/after-frame";

const app = {
  baseUrl: {
    angular: "http://localhost:3000",
    leptos: "http://localhost:3001",
    react: "http://localhost:3002",
    vue: "http://localhost:3003",
  },
}

const baseUrl = (path: string)  => `${app.baseUrl.vue}${path}`

test.describe('initial rendering phase', () => {

  test('warm up phase', async ({page}) => {
    // Ignore analytics when warming up
    await page.route('http://localhost:8000/api/v1/event', (route) => {
      route.fulfill({ status: 204 });
    });

    await page.addInitScript({
      // TODO: parse script and replace project name, environment and version
      path: resolve(__dirname, '../utils/web-vitals.js')
    });

    // open page
    await page.goto(baseUrl("/"), {
      timeout: 1000, // 1 second
      waitUntil: 'networkidle'
    })

    // TODO: Validate elements are available in the DOM
    // await page.waitForSelector('h1')

    // click on an element to get Web Vitals like INP
    await page.getByRole('heading').click()
  })

  test('test phase', async ({page}) => {
    const n = 100;

    for (let i = 0; i < n; i++) {
      await page.goto(baseUrl("/"))
      await page.getByRole('heading').click()
    }
  })
})

test.describe('responsiveness', () => {

  test('warm up phase', async ({ page }) => {
    // Ignore analytics when warming up
    await page.route('http://localhost:8000/api/v1/event', (route) => {
      route.fulfill({ status: 204 });
    });

    // Add custom script to measure frame rate
    await page.addInitScript(afterFrame)

    // open page
    await page.goto(baseUrl("/"), {
      timeout: 1000, // 1 second
      waitUntil: 'networkidle'
    })

    // TODO: Validate elements are available in the DOM

    // Create 1000 rows
    await page.evaluate( () => {
      performance.mark("btn:run_start");
      document.getElementById("run").click()
      // @ts-ignore: We are sure that the function is available in the window object
      window.afterFrame(() => {
        performance.mark("btn:run_end");
      })
    });

    // wait for table to load
    await page
      .locator('.table > tbody:nth-child(1) > tr:nth-child(1000) > td:nth-child(1)')
      .waitFor({
        state: 'attached',
        timeout: 1000
      });

    // Calculate the duration of the action
    const duration = await page.evaluate(() => {
      const measure = performance.measure("btn:run_duration", "btn:run_start", "btn:run_end");
      return measure.duration;
    });
    console.log("duration:", duration)
  })

  test('test phase', async ({page}) => {
    // Enforce garbage collection
    await page.evaluate("window.gc({type:'major',execution:'sync',flavor:'last-resort'})");

    const client = await page.context().newCDPSession(page);
    // Emulate slow CPU
    await client.send("Emulation.setCPUThrottlingRate", {
      rate: 3
    });

    // await browser.startTracing(page, {
    //   path: fileNameTrace(framework, benchmark.benchmarkInfo, i, benchmarkOptions),
    //   screenshots: false,
    //   categories: categories,
    // });

    // RUN BENCHMARK

    // await wait(40);
    // await browser.stopTracing();
  })
})


test.describe('experimental: chromium only', () => {
  test.skip(({ browserName }) => browserName !== 'chromium', 'Chromium only!');

  test('Get performance metrics', async ({page}) => {
    // Create a new connection to an existing CDP session to enable performance Metrics
    const session = await page.context().newCDPSession(page)
    // Start record CDP session performance metrics
    await session.send("Performance.enable")
    await page.goto("/")
    const performanceMetrics = await session.send("Performance.getMetrics")

    console.log(performanceMetrics.metrics)
  })

  test("Capture performance traces by marking actions using Performance API", async ({page, browser}) => {
    await browser.startTracing(page, {path: './test-results/perf-profile.json', screenshots: true})
    await page.goto("/")

    // Using Performance.mark API
    await page.evaluate(() => window.performance.mark('home:heading:click_start'))
    await page.getByRole('heading', {name: 'Hello World'}).click()
    await page.evaluate(() => window.performance.mark('home:heading:click_end'))

    // To get all performance marks
    const getAllMarksJson = await page.evaluate(
      () => JSON.stringify(window.performance.getEntriesByType("mark")
      ))
    console.log('Performance API:', JSON.parse(getAllMarksJson))


    // Performance measure
    await page.evaluate(
      () => window.performance.measure("home:heading:click_duration", "home:heading:click_start", "home:heading:click_end")
    )

    // To get all performance measures of Google
    const getAllMeasuresJson = await page.evaluate(
      () => JSON.stringify(window.performance.getEntriesByType("measure"))
    )
    console.log('Performance API:', JSON.parse(getAllMeasuresJson))

    await browser.stopTracing()
  })
});


