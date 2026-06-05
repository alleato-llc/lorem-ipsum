// Entry point: fetch backend data, wire the form and output modules, run
// the first generation.

import { fetchSettings, fetchThemes, requestGeneration } from "./api";
import { currentOptions, initForm } from "./form";
import { initOutput, render } from "./output";

async function generate(): Promise<void> {
  render(await requestGeneration(currentOptions()));
}

async function init(): Promise<void> {
  const [themes, saved] = await Promise.all([fetchThemes(), fetchSettings()]);
  initForm(themes, saved, () => void generate());
  initOutput();
  await generate();
}

void init();
