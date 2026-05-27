interface TauriInvoke {
  getVersion: () => Promise<string>;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const TauriWindow = window as any;

export function useTauri(): TauriInvoke {
  const invoke = TauriWindow.__TAURI__?.invoke;

  return {
    async getVersion(): Promise<string> {
      if (!invoke) return '0.1.0 (dev)';
      try {
        return await invoke('get_version');
      } catch {
        return 'unknown';
      }
    },
  };
}
