import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Toolbar } from './Toolbar';

describe('Toolbar', () => {
  it('renders app title', () => {
    render(<Toolbar engineStatus="connected" />);
    expect(screen.getByText('KICKS')).toBeInTheDocument();
    expect(screen.getByText('Guitar Workstation')).toBeInTheDocument();
  });

  it('shows green dot when connected', () => {
    render(<Toolbar engineStatus="connected" />);
    expect(screen.getByText('RUNNING')).toBeInTheDocument();
    const container = screen.getByText('RUNNING').parentElement;
    const dot = container?.querySelector('span');
    expect(dot?.className).toContain('bg-[var(--success)]');
  });

  it('shows yellow dot when connecting', () => {
    render(<Toolbar engineStatus="connecting" />);
    expect(screen.getByText('CONNECTING')).toBeInTheDocument();
    const container = screen.getByText('CONNECTING').parentElement;
    const dot = container?.querySelector('span');
    expect(dot?.className).toContain('bg-[var(--warning)]');
  });

  it('shows red dot when disconnected', () => {
    render(<Toolbar engineStatus="disconnected" />);
    expect(screen.getByText('STOPPED')).toBeInTheDocument();
    const container = screen.getByText('STOPPED').parentElement;
    const dot = container?.querySelector('span');
    expect(dot?.className).toContain('bg-[var(--danger)]');
  });
});
