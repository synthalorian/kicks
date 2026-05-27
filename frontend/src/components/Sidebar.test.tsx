import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Sidebar } from './Sidebar';

describe('Sidebar', () => {
  it('renders all navigation items', () => {
    render(<Sidebar activePage="signal-chain" onNavigate={() => {}} />);
    expect(screen.getByText('Signal Chain')).toBeInTheDocument();
    expect(screen.getByText('Presets')).toBeInTheDocument();
    expect(screen.getByText('IR Browser')).toBeInTheDocument();
    expect(screen.getByText('MIDI')).toBeInTheDocument();
    expect(screen.getByText('Live')).toBeInTheDocument();
    expect(screen.getByText('AI Assistant')).toBeInTheDocument();
    expect(screen.getByText('Settings')).toBeInTheDocument();
  });

  it('highlights the active page', () => {
    render(<Sidebar activePage="presets" onNavigate={() => {}} />);
    const activeBtn = screen.getByText('Presets').closest('button');
    expect(activeBtn?.className).toContain('bg-[var(--accent)]');
  });

  it('calls onNavigate when a page is clicked', () => {
    const onNavigate = vi.fn();
    render(<Sidebar activePage="signal-chain" onNavigate={onNavigate} />);

    fireEvent.click(screen.getByText('MIDI'));
    expect(onNavigate).toHaveBeenCalledWith('midi');

    fireEvent.click(screen.getByText('Settings'));
    expect(onNavigate).toHaveBeenCalledWith('settings');
  });

  it('calls onNavigate even when active page is clicked', () => {
    const onNavigate = vi.fn();
    render(<Sidebar activePage="signal-chain" onNavigate={onNavigate} />);

    fireEvent.click(screen.getByText('Signal Chain'));
    expect(onNavigate).toHaveBeenCalledWith('signal-chain');
  });
});
