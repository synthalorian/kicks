interface TauriInvoke {
  getVersion: () => Promise<string>;
}

export function useTauri(): TauriInvoke {
  const invoke = (window as any).__TAURI__?.invoke;

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
