import { useEffect, useRef } from 'react';

export interface HotkeyDef {
  /** KeyboardEvent.key value (e.g. 's', 'Escape', ' ', 'Enter') */
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  meta?: boolean;
  handler: (e: KeyboardEvent) => void;
  /**
   * When true, the shortcut won't fire if focus is inside an input,
   * textarea, select, or contenteditable element. Default: false.
   */
  ignoreInput?: boolean;
}

/**
 * Register keyboard shortcuts in any component.
 *
 * Hotkeys are matched by key + modifier combination. The first matching
 * shortcut fires. Handlers are compared by reference so the hook can
 * efficiently update without full re-registration.
 *
 * @example
 * ```tsx
 * useHotkeys([
 *   { key: 's', ctrl: true, handler: handleSave, ignoreInput: true },
 *   { key: 'Escape', handler: closeDialog },
 * ]);
 * ```
 */
export function useHotkeys(hotkeys: HotkeyDef[]): void {
  // Keep a ref so the effect doesn't re-subscribe on every render.
  const hotkeysRef = useRef(hotkeys);

  useEffect(() => {
    hotkeysRef.current = hotkeys;
  }, [hotkeys]);

  useEffect(() => {
    const listener = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement | null;
      const inInput =
        target &&
        (target.tagName === 'INPUT' ||
          target.tagName === 'TEXTAREA' ||
          target.tagName === 'SELECT' ||
          target.isContentEditable);

      for (const hk of hotkeysRef.current) {
        const matchKey = e.key.toLowerCase() === hk.key.toLowerCase();
        const matchCtrl = !!hk.ctrl === (e.ctrlKey || e.metaKey);
        const matchShift = !!hk.shift === e.shiftKey;
        const matchAlt = !!hk.alt === e.altKey;

        if (!matchKey || !matchCtrl || !matchShift || !matchAlt) continue;

        if (hk.ignoreInput && inInput) continue;

        e.preventDefault();
        e.stopPropagation();
        hk.handler(e);
        break;
      }
    };

    window.addEventListener('keydown', listener, { capture: true });
    return () => window.removeEventListener('keydown', listener, { capture: true });
  }, []);
}
