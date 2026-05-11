#!/bin/sh
# Sprout installer for Linux and macOS.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/JosaaXc/sprout/main/install.sh | sh
#
# Optional flags (pass after `sh -s --`):
#   --version vX.Y.Z   Install a specific tag (default: latest)
#   --dir DIR          Install into DIR (default: $HOME/.local/bin)
#
# Example:
#   curl -fsSL https://raw.githubusercontent.com/JosaaXc/sprout/main/install.sh | sh -s -- --version v0.1.0

set -eu

REPO="JosaaXc/sprout"
BIN_NAME="sprout"
VERSION="latest"
INSTALL_DIR="${SPROUT_INSTALL_DIR:-$HOME/.local/bin}"

while [ $# -gt 0 ]; do
    case "$1" in
        --version) VERSION="$2"; shift 2 ;;
        --dir)     INSTALL_DIR="$2"; shift 2 ;;
        -h|--help)
            sed -n '2,12p' "$0" | sed 's/^# \{0,1\}//'
            exit 0
            ;;
        *) echo "Unknown argument: $1" >&2; exit 1 ;;
    esac
done

# --- detect OS ---
case "$(uname -s)" in
    Linux)  os_id="unknown-linux-gnu" ;;
    Darwin) os_id="apple-darwin" ;;
    *) echo "Unsupported OS: $(uname -s). See https://github.com/${REPO}/releases for manual install." >&2; exit 1 ;;
esac

# --- detect arch ---
case "$(uname -m)" in
    x86_64|amd64)   arch_id="x86_64" ;;
    arm64|aarch64)  arch_id="aarch64" ;;
    *) echo "Unsupported architecture: $(uname -m). See https://github.com/${REPO}/releases for manual install." >&2; exit 1 ;;
esac

target="${arch_id}-${os_id}"
archive="${BIN_NAME}-${target}.tar.gz"

# --- resolve version ---
if [ "$VERSION" = "latest" ]; then
    VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep -oE '"tag_name":[[:space:]]*"[^"]+"' \
        | head -1 \
        | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "${VERSION:-}" ]; then
        echo "Could not resolve the latest release tag from GitHub." >&2
        echo "Check https://github.com/${REPO}/releases or pass --version vX.Y.Z." >&2
        exit 1
    fi
fi

url="https://github.com/${REPO}/releases/download/${VERSION}/${archive}"

echo "→ Installing sprout ${VERSION} (${target})"
echo "  from ${url}"

tmpdir="$(mktemp -d 2>/dev/null || mktemp -d -t sprout-install)"
trap 'rm -rf "$tmpdir"' EXIT INT TERM

if ! curl -fSL --progress-bar "$url" -o "$tmpdir/$archive"; then
    echo "Download failed. Verify that ${VERSION} has assets at https://github.com/${REPO}/releases" >&2
    exit 1
fi

echo "→ Extracting..."
tar -xzf "$tmpdir/$archive" -C "$tmpdir"

mkdir -p "$INSTALL_DIR"
mv "$tmpdir/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

echo
echo "✓ sprout installed to ${INSTALL_DIR}/${BIN_NAME}"

# --- PATH check ---
case ":${PATH:-}:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        echo
        echo "ℹ ${INSTALL_DIR} is not on your PATH."
        echo "  Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo
        echo "      export PATH=\"${INSTALL_DIR}:\$PATH\""
        ;;
esac

echo
"$INSTALL_DIR/$BIN_NAME" --version 2>/dev/null || true
