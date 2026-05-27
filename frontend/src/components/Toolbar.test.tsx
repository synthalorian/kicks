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
    expect(screen.getByText('Engine Running')).toBeInTheDocument();
    // The dot is a span with bg-green-500 class
    const dot = document.querySelector('.bg-green-500');
    expect(dot).toBeInTheDocument();
  });

  it('shows yellow dot when connecting', () => {
    render(<Toolbar engineStatus="connecting" />);
    expect(screen.getByText('Connecting...')).toBeInTheDocument();
    const dot = document.querySelector('.bg-yellow-500');
    expect(dot).toBeInTheDocument();
  });

  it('shows red dot when disconnected', () => {
    render(<Toolbar engineStatus="disconnected" />);
    expect(screen.getByText('Engine Stopped')).toBeInTheDocument();
    const dot = document.querySelector('.bg-red-500');
    expect(dot).toBeInTheDocument();
  });
});
