#!/usr/bin/env bash
# Install velo from GitHub Releases.
# Usage: install.sh [version]     (default: latest)
# Env:   INSTALL_DIR  install location (default: $HOME/.velo/bin)
set -euo pipefail

REPO="BazilSuhail/Velocity"
BINARY="velo"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.velo/bin}"
VERSION="${1:-latest}"

die() { echo "error: $*" >&2; exit 1; }

# Detect OS and architecture
case "$(uname -s)" in
  Linux)
    case "$(uname -m)" in
      x86_64|amd64)  TARGET="x86_64-unknown-linux-gnu" ;;
      aarch64|arm64) TARGET="aarch64-unknown-linux-gnu" ;;
      *) die "unsupported Linux architecture: $(uname -m)" ;;
    esac
    ;;
  Darwin)
    case "$(uname -m)" in
      arm64|aarch64) TARGET="aarch64-apple-darwin" ;;
      x86_64)        TARGET="x86_64-apple-darwin" ;;
      *) die "unsupported macOS architecture: $(uname -m)" ;;
    esac
    ;;
  *) die "unsupported OS: $(uname -s). On Windows, use scripts/install.ps1" ;;
esac

# Resolve the release tag
if [ "$VERSION" = "latest" ]; then
  TAG="$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
    | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')" \
    || die "could not query the latest release (check your network)"
  [ -n "$TAG" ] || die "could not determine the latest release tag"
else
  TAG="v$VERSION"
fi

ARCHIVE="$BINARY-$TARGET.tar.gz"
URL="https://github.com/$REPO/releases/download/$TAG/$ARCHIVE"

# Private temp dir, removed on exit (success or failure)
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Downloading $URL"
curl -fsSL "$URL" -o "$TMP_DIR/$ARCHIVE" \
  || die "download failed: $URL (check the version and your network)"
tar -xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR"
[ -f "$TMP_DIR/$BINARY" ] || die "archive did not contain $BINARY"

mkdir -p "$INSTALL_DIR"
mv -f "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"
chmod +x "$INSTALL_DIR/$BINARY"

# Add to PATH in shell rc files, once per file
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    LINE="export PATH=\"\$PATH:$INSTALL_DIR\""
    RC_FILES=()
    for RC in "$HOME/.bashrc" "$HOME/.zshrc"; do
      [ -f "$RC" ] && RC_FILES+=("$RC")
    done
    [ ${#RC_FILES[@]} -gt 0 ] || RC_FILES=("$HOME/.bashrc")
    for RC in "${RC_FILES[@]}"; do
      if grep -qsF "$INSTALL_DIR" "$RC"; then
        continue
      fi
      printf '\n# Added by velo installer\n%s\n' "$LINE" >> "$RC"
      echo "Added $INSTALL_DIR to PATH in $RC"
    done
    ;;
esac

echo ""
echo "velo $TAG installed successfully."
echo "  Binary: $INSTALL_DIR/$BINARY"
echo ""
echo "Open a new terminal (or run: export PATH=\"\$PATH:$INSTALL_DIR\"), then run: velo --help"
