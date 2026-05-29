import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { StatusBar } from './StatusBar';

vi.mock('../stores/engineStore', () => ({
  useEngineStore: (selector: (s: Record<string, unknown>) => unknown) =>
    selector({
      cpuLoad: 0,
      status: { running: true, sample_rate: 48000, buffer_size: 256 },
      pollCpu: vi.fn(),
    }),
}));

describe('StatusBar', () => {
  it('renders version string', () => {
    render(<StatusBar version="0.5.2" engineStatus="connected" />);
    expect(screen.getByText('v0.5.2')).toBeInTheDocument();
  });

  it('shows connected status', () => {
    render(<StatusBar version="1.0.0" engineStatus="connected" />);
    expect(screen.getByText('ENGINE ACTIVE')).toBeInTheDocument();
  });

  it('shows disconnected status', () => {
    render(<StatusBar version="1.0.0" engineStatus="disconnected" />);
    expect(screen.getByText('ENGINE INACTIVE')).toBeInTheDocument();
  });
});
