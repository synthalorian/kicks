#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────────────────────────
# build-all.sh — Build all Kicks packages
#
# Runs: cargo build, npm build, tests, and packaging
# Usage:
#   ./scripts/build-all.sh              # dev build + test
#   ./scripts/build-all.sh --release    # release build + all packages
#   ./scripts/build-all.sh --quick      # just cargo build + test (skip frontend)
# ──────────────────────────────────────────────────────────────────────────────
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "${ROOT}"

RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

pass() { printf " [${GREEN}OK${NC}]\n"; }
fail() { printf " [${RED}FAIL${NC}]\n"; exit 1; }
step() { printf "${CYAN}─── %s${NC}\n" "$*"; }

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Kicks — Full Build"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# ── Rust ──
step "Rust: cargo check"
cargo check 2>&1 | tail -1 || fail
pass

# ── Tests ──
step "Rust: cargo test"
cargo test 2>&1 | tail -5 || fail
pass

# ── Frontend ──
if [[ "${1:-}" != "--quick" ]]; then
  step "Frontend: npm ci"
  (cd frontend && npm ci) || fail
  pass

  step "Frontend: tsc --noEmit"
  (cd frontend && npx tsc --noEmit) || fail
  pass

  step "Frontend: npm run build"
  (cd frontend && npm run build) || fail
  pass
fi

# ── Release build ──
if [[ "${1:-}" == "--release" ]]; then
  step "Rust: cargo build --release"
  cargo build --release 2>&1 | tail -1 || fail
  pass

  # Tauri bundle (AppImage + .deb)
  step "Tauri: bundle AppImage + deb"
  (cd src-tauri && cargo tauri build) 2>&1 | tail -5 || fail
  pass

  # Flatpak
  if command -v flatpak-builder &>/dev/null; then
    step "Flatpak: build"
    ./scripts/build-flatpak.sh || echo "  ⚠️  Flatpak build skipped (missing deps?)"
  else
    echo "  - flatpak-builder not found, skipping Flatpak"
  fi

  # Snap
  if command -v snapcraft &>/dev/null; then
    step "Snap: build"
    ./scripts/build-snap.sh --destructive || echo "  ⚠️  Snap build skipped"
  else
    echo "  - snapcraft not found, skipping Snap"
  fi
fi

echo "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "${GREEN}  All builds passed${NC}"
echo "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
