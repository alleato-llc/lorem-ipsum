// End-to-end: the full app in a real browser (mock backend), exercising the
// form → generate → render loop the way a user would.

import { expect, test } from "@playwright/test";

test("loads, generates on startup, and shows stats", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("#output p")).not.toHaveCount(0);
  await expect(page.locator("#theme option")).toHaveCount(5);
  await expect(page.locator("#stats")).toContainText("seed");
  await expect(page.locator("#copy")).toBeEnabled();
});

test("changing theme updates the description", async ({ page }) => {
  await page.goto("/");
  await page.locator("#theme").selectOption("pirate");
  await expect(page.locator("#theme-description")).toContainText("High-seas");
  await expect(page.locator("#start-with-lorem")).toBeDisabled();
});

test("words mode hides ranges and retunes the count slider", async ({ page }) => {
  await page.goto("/");
  await page.locator("#mode").selectOption("words");
  await expect(page.locator("#sentence-range")).toBeHidden();
  await expect(page.locator("#word-range")).toBeHidden();
  await expect(page.locator("#count-label")).toHaveText("Words");
  await expect(page.locator("#count-out")).toHaveText("50");

  await page.locator("#options-form button[type=submit]").click();
  await expect(page.locator("#output")).toHaveClass(/words-mode/);
  await expect(page.locator("#output p")).toHaveCount(1);
});

test("the same seed reproduces the same output", async ({ page }) => {
  await page.goto("/");
  await page.locator("#seed").fill("42");
  await page.locator("#options-form button[type=submit]").click();
  await expect(page.locator("#stats")).toContainText("seed 42");
  const first = await page.locator("#output").innerText();

  await page.locator("#options-form button[type=submit]").click();
  await expect(page.locator("#stats")).toContainText("seed 42");
  const second = await page.locator("#output").innerText();

  expect(second).toBe(first);
});

test("paragraph count follows the slider", async ({ page }) => {
  await page.goto("/");
  await page.locator("#seed").fill("7");
  await page.locator("#count").fill("5");
  await page.locator("#options-form button[type=submit]").click();
  await expect(page.locator("#output p")).toHaveCount(5);
});
