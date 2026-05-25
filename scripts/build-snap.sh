#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────────────────────────
# build-snap.sh — Build Kicks Snap package
#
# Prerequisites:
#   snapcraft    (install: sudo snap install snapcraft --classic)
#   lxd          (for clean build environments)
#   or use:      snapcraft --destructive-mode  (builds on host)
#
# Usage:
#   ./scripts/build-snap.sh              # clean build via LXD
#   ./scripts/build-snap.sh --destructive # build on host (faster, risky)
#   ./scripts/build-snap.sh --help       # show Snapcraft help
# ──────────────────────────────────────────────────────────────────────────────
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SNAP_DIR="${ROOT}/snap"
OUTPUT_DIR="${ROOT}/dist"

cd "${ROOT}"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Kicks Snap Builder"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

case "${1:-}" in
  --destructive)
    echo "⚠️  Building on host (destructive mode)"
    echo "   Make sure build deps are installed:"
    echo "   libasound2-dev libjack-jackd2-dev libwebkit2gtk-4.1-dev"
    echo "   libgtk-3-dev libayatana-appindicator3-dev nodejs npm rustc cargo"
    echo ""
    mkdir -p "${OUTPUT_DIR}"
    snapcraft --destructive-mode --output "${OUTPUT_DIR}/kicks.snap"
    echo "✅ Done: ${OUTPUT_DIR}/kicks.snap"
    ;;
  --help|--help-all)
    snapcraft help
    ;;
  *)
    echo "→ Building in clean LXD container..."
    mkdir -p "${OUTPUT_DIR}"
    snapcraft --output "${OUTPUT_DIR}/kicks.snap"
    echo "✅ Done: ${OUTPUT_DIR}/kicks.snap"
    ;;
esac
