# Component tests

DOM-level tests for the frontend's `form.ts` and `output.ts`, run by vitest
in a happy-dom environment: `gui/tests/component/*.test.ts`.

## Real markup, not fixtures

Tests run against the markup the app actually ships.
`tests/helpers/fixture.ts` reads `index.html`, strips scripts, and mounts
the body:

```ts
import { mountApp } from "../helpers/fixture";

beforeEach(() => {
  mountApp(); // fresh copy of the real #app markup
});
```

If `index.html` changes shape, these tests notice — that's the point.

## Patterns

- **Fresh DOM per test** works because the source modules look elements up
  through `dom.ts`'s `byId` on every call instead of caching references at
  import time. Keep it that way.
- **Drive through events**, not internals: set `el.value` then dispatch
  `new Event("change", { bubbles: true })`; submit via
  `new Event("submit", { bubbles: true, cancelable: true })`.
- **Stub browser APIs** that happy-dom lacks or that have side effects:
  `vi.stubGlobal("navigator", { clipboard: { writeText } })` for the copy
  button, with `vi.unstubAllGlobals()` after.

## What's covered

- `initForm` populates the theme select and applies saved defaults.
- Theme changes update the description and gate the classic-opener
  checkbox.
- Mode changes hide inapplicable range fields and retune the count
  slider's bounds/label/value (including clamping out-of-range saved
  counts).
- Submit fires the generate callback (and is prevented from reloading).
- `currentOptions()` reflects live form state.
- `render` produces one `<p>` per item, accurate stats (seed included),
  the `words-mode` class toggle, and enables Copy.
- Copy writes items joined by blank lines to the clipboard.

## A caveat these tests taught us

Component tests assert DOM *properties* (e.g. `el.hidden === true`) — they
cannot see whether CSS actually hides the element. The e2e layer caught a
case where author CSS overrode `[hidden]`; see [e2e.md](e2e.md). When a
behavior depends on computed style, cover it end-to-end.
