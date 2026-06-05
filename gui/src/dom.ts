// Element lookup helper. Queried fresh on every call so modules hold no
// stale references — important for component tests that rebuild the DOM.

export const byId = <T extends HTMLElement>(id: string): T =>
  document.getElementById(id) as T;
