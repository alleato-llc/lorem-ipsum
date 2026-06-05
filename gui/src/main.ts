// Entry point: fetch backend data, wire the form and output modules, run
// the first generation.

import { backend } from "./backend";
import { currentOptions, initForm } from "./form";
import { initOutput, render } from "./output";

async function generate(): Promise<void> {
  render(await backend.generate(currentOptions()));
}

async function init(): Promise<void> {
  const [themes, saved] = await Promise.all([backend.themes(), backend.settings()]);
  initForm(themes, saved, () => void generate());
  initOutput();
  await generate();
}

void init();
