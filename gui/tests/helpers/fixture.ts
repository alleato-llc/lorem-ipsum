// Loads the real index.html markup into the test DOM so component tests
// run against the same structure the app ships with.

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));

/** Body markup from index.html, scripts stripped. */
export function appMarkup(): string {
  const html = readFileSync(resolve(here, "../../index.html"), "utf-8");
  const body = html.match(/<body>([\s\S]*)<\/body>/)?.[1] ?? "";
  return body.replace(/<script[\s\S]*?<\/script>/g, "");
}

/** Reset the document to a fresh copy of the app markup. */
export function mountApp(): void {
  document.body.innerHTML = appMarkup();
}
