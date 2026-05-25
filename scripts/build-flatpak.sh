#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────────────────────────
# build-flatpak.sh — Build Kicks Flatpak package
#
# Prerequisites:
#   flatpak       (runtime)
#   flatpak-builder  (build tool)
#   org.freedesktop.Platform//24.08
#   org.freedesktop.Sdk//24.08
#   org.freedesktop.Sdk.Extension.rust-stable//24.08
#   org.freedesktop.Sdk.Extension.node22//24.08
#
# Usage:
#   ./scripts/build-flatpak.sh          # default release build
#   ./scripts/build-flatpak.sh --debug  # debug build (unoptimized, verbose)
# ──────────────────────────────────────────────────────────────────────────────
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FLATPAK_DIR="${ROOT}/flatpak"
MANIFEST="${FLATPAK_DIR}/com.kicks.guitar-workstation.yml"
BUILD_DIR="${ROOT}/build/flatpak"
REPO_DIR="${ROOT}/build/flatpak-repo"
EXPORT_DIR="${ROOT}/dist"

FLAGS=(
  --force-clean
  --ccache
  --install-deps-from=flathub
  --repo="${REPO_DIR}"
  "${BUILD_DIR}"
  "${MANIFEST}"
)

if [[ "${1:-}" == "--debug" ]]; then
  FLAGS=(--build-type=debug --verbose "${FLAGS[@]}")
  echo "🔧 Debug build"
else
  echo "📦 Release build"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Kicks Flatpak Builder"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# ── Ensure runtimes are installed ──
for RUNTIME in \
  org.freedesktop.Platform//24.08 \
  org.freedesktop.Sdk//24.08 \
  org.freedesktop.Sdk.Extension.rust-stable//24.08 \
  org.freedesktop.Sdk.Extension.node22//24.08; do
  if ! flatpak info "${RUNTIME}" &>/dev/null; then
    echo "→ Installing ${RUNTIME}..."
    flatpak install --noninteractive flathub "${RUNTIME}"
  fi
done

# ── Build ──
echo "→ Building Flatpak..."
mkdir -p "${BUILD_DIR}" "${EXPORT_DIR}"
flatpak-builder "${FLAGS[@]}"

# ── Export to .flatpak file ──
echo "→ Exporting .flatpak bundle..."
flatpak build-bundle \
  --runtime-repo="https://flathub.org/repo/flathub.flatpakrepo" \
  "${REPO_DIR}" \
  "${EXPORT_DIR}/Kicks.flatpak" \
  com.kicks.guitar-workstation

echo "✅ Done: ${EXPORT_DIR}/Kicks.flatpak"
