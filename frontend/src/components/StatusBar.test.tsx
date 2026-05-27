import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { StatusBar } from './StatusBar';

describe('StatusBar', () => {
  it('renders version string', () => {
    render(<StatusBar version="0.5.2" engineStatus="connected" />);
    expect(screen.getByText('Kicks v0.5.2')).toBeInTheDocument();
  });

  it('shows connected status', () => {
    render(<StatusBar version="1.0.0" engineStatus="connected" />);
    const jackLabel = screen.getByText('Connected');
    expect(jackLabel).toBeInTheDocument();
    expect(jackLabel.className).toContain('text-green-400');
  });

  it('shows disconnected status', () => {
    render(<StatusBar version="1.0.0" engineStatus="disconnected" />);
    const jackLabel = screen.getByText('Not connected');
    expect(jackLabel).toBeInTheDocument();
    expect(jackLabel.className).toContain('text-red-400');
  });
});
