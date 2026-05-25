import { useState, useEffect, useCallback, type ReactNode } from 'react';
import { ErrorBoundary } from './components/ErrorBoundary';
import { Toolbar } from './components/Toolbar';
import { Sidebar, type Page } from './components/Sidebar';
import { StatusBar } from './components/StatusBar';
import { SignalChain } from './pages/SignalChain';
import { Presets } from './pages/Presets';
import { IRBrowser } from './pages/IRBrowser';
import { MidiConfig } from './pages/MidiConfig';
import { LiveMode } from './pages/LiveMode';
import { AIAssistant } from './pages/AIAssistant';
import { Settings } from './pages/Settings';
import { useEngineStore } from './stores/engineStore';
import { getVersion } from './lib/tauri';
import { useHotkeys, type HotkeyDef } from './hooks/useHotkeys';

const pages: Record<Page, () => ReactNode> = {
  'signal-chain': SignalChain,
  presets: Presets,
  'ir-browser': IRBrowser,
  midi: MidiConfig,
  live: LiveMode,
  'ai-assistant': AIAssistant,
  settings: Settings,
};

function App() {
  const [activePage, setActivePage] = useState<Page>('signal-chain');
  const [appVersion, setAppVersion] = useState('0.1.0 (dev)');
  const status = useEngineStore((s) => s.status);
  const fetchStatus = useEngineStore((s) => s.fetchStatus);
  const start = useEngineStore((s) => s.start);
  const stop = useEngineStore((s) => s.stop);

  useEffect(() => {
    fetchStatus();
    getVersion().then(setAppVersion).catch(() => {});
  }, [fetchStatus]);

  // ── Global keyboard shortcuts ──

  const pageKeys: Record<string, Page> = {
    '1': 'signal-chain',
    '2': 'presets',
    '3': 'ir-browser',
    '4': 'midi',
    '5': 'live',
    '6': 'ai-assistant',
    '7': 'settings',
  };

  const handleEngineToggle = useCallback(() => {
    if (status.running) stop();
    else start();
  }, [status.running, start, stop]);

  const undo = useEngineStore((s) => s.undo);
  const redo = useEngineStore((s) => s.redo);

  const handleUndo = useCallback(() => undo(), [undo]);
  const handleRedo = useCallback(() => redo(), [redo]);

  const globalHotkeys: HotkeyDef[] = [
    ...Object.entries(pageKeys).map(([key, page]) => ({
      key,
      handler: () => setActivePage(page),
      ignoreInput: true,
    })),
    {
      key: ' ',
      handler: handleEngineToggle,
      ignoreInput: true,
    },
    { key: 'z', ctrl: true, handler: handleUndo, ignoreInput: true },
    { key: 'z', ctrl: true, shift: true, handler: handleRedo, ignoreInput: true },
  ];

  useHotkeys(globalHotkeys);

  const engineIndicator: 'disconnected' | 'connecting' | 'connected' = status.running
    ? 'connected'
    : 'disconnected';

  const PageComponent = pages[activePage];

  return (
    <div className="h-full flex flex-col">
      <Toolbar engineStatus={engineIndicator} />
      <main className="flex-1 flex overflow-hidden">
        <Sidebar activePage={activePage} onNavigate={setActivePage} />
        <section className="flex-1 p-6 overflow-y-auto">
          <ErrorBoundary>
            <PageComponent />
          </ErrorBoundary>
        </section>
      </main>
      <StatusBar version={appVersion} engineStatus={engineIndicator} />
    </div>
  );
}

export default App;
