import { vi } from 'vitest';

// Node puede exponer su propio localStorage antes de que Vitest copie el de JSDOM.
const { jsdom } = globalThis as typeof globalThis & {
  jsdom: { window: Pick<Window, 'localStorage'> };
};

vi.stubGlobal('localStorage', jsdom.window.localStorage);
