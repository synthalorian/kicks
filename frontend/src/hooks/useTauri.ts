interface TauriInvoke {
  getVersion: () => Promise<string>;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const TauriWindow = window as any;

export function useTauri(): TauriInvoke {
  // Tauri v2: __TAURI__.core.invoke; v1: __TAURI__.invoke
  const invoke = TauriWindow.__TAURI__?.invoke ?? TauriWindow.__TAURI__?.core?.invoke;

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
