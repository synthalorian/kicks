import { useState, useEffect, useCallback, useRef, type ReactNode } from 'react';
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
import { Tools } from './pages/Tools';
import { NAMBrowser } from './pages/NAMBrowser';
import { SetupWizard } from './components/SetupWizard';
import { useEngineStore } from './stores/engineStore';
import { getVersion } from './lib/tauri';
import { useHotkeys, type HotkeyDef } from './hooks/useHotkeys';
import { applyTheme, getSavedThemeId, getThemeById } from './theme/theme';

const pages: Record<Page, () => ReactNode> = {
  'signal-chain': SignalChain,
  presets: Presets,
  'ir-browser': IRBrowser,
  'nam-browser': NAMBrowser,
  midi: MidiConfig,
  live: LiveMode,
  tools: Tools,
  'ai-assistant': AIAssistant,
  settings: Settings,
};

function App() {
  const [activePage, setActivePage] = useState<Page>('signal-chain');
  const [appVersion, setAppVersion] = useState('0.1.0');
  const [showWizard, setShowWizard] = useState(false);
  const status = useEngineStore((s) => s.status);
  const isTauri = useEngineStore((s) => s.isTauri);
  const fetchStatus = useEngineStore((s) => s.fetchStatus);
  const start = useEngineStore((s) => s.start);
  const stop = useEngineStore((s) => s.stop);
  const isStartingRef = useRef(false);

  // Initialize theme on mount
  useEffect(() => {
    const savedId = getSavedThemeId();
    applyTheme(getThemeById(savedId));
  }, []);

  useEffect(() => {
    fetchStatus();
    getVersion().then((v) => setAppVersion(v)).catch(() => {});
  }, [fetchStatus]);

  // Show wizard on first launch
  useEffect(() => {
    const dismissed = localStorage.getItem('kicks:wizard-dismissed');
    if (!dismissed) {
      setShowWizard(true);
    }
  }, []);

  // Auto-start engine in browser mode so the UI isn't dead
  useEffect(() => {
    if (!isTauri && !status.running && !isStartingRef.current) {
      isStartingRef.current = true;
      (async () => {
        try {
          await start();
        } catch {
          // Browser might block autoplay — user can manually start
        } finally {
          isStartingRef.current = false;
        }
      })();
    }
  }, [isTauri, status.running, start]);

  // ── Global keyboard shortcuts ──
  const pageKeys: Record<string, Page> = {
    '1': 'signal-chain',
    '2': 'presets',
    '3': 'ir-browser',
    '4': 'nam-browser',
    '5': 'midi',
    '6': 'live',
    '7': 'tools',
    '8': 'ai-assistant',
    '9': 'settings',
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

  const modeLabel = isTauri ? 'DESKTOP' : 'BROWSER';

  const PageComponent = pages[activePage];

  return (
    <div className="h-full flex flex-col grid-bg">
      {showWizard && <SetupWizard onClose={() => setShowWizard(false)} />}
      <Toolbar engineStatus={engineIndicator} modeLabel={modeLabel} />
      <main className="flex-1 flex overflow-hidden">
        <Sidebar activePage={activePage} onNavigate={setActivePage} onShowWizard={() => setShowWizard(true)} />
        <section className="flex-1 overflow-y-auto p-5">
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
