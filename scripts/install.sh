#!/usr/bin/env bash
set -euo pipefail

REPO="BazilSuhail/Velocity"
BINARY="velo"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.velo/bin}"
VERSION="${1:-latest}"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)  TARGET="x86_64-unknown-linux-gnu" ;;
  Darwin)
    case "$ARCH" in
      arm64) TARGET="aarch64-apple-darwin" ;;
      *)     TARGET="x86_64-apple-darwin" ;;
    esac
    ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Resolve latest version
if [ "$VERSION" = "latest" ]; then
  API_URL="https://api.github.com/repos/$REPO/releases/latest"
  TAG="$(curl -sL "$API_URL" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": "//;s/".*//')"
else
  TAG="v$VERSION"
fi

ARCHIVE="$BINARY-$TARGET.tar.gz"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$TAG/$ARCHIVE"

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download and extract
echo "Downloading $DOWNLOAD_URL ..."
curl -sL "$DOWNLOAD_URL" -o "/tmp/$ARCHIVE"
tar xzf "/tmp/$ARCHIVE" -C "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/$BINARY"
rm -f "/tmp/$ARCHIVE"

# Add to PATH if not already there
case ":${PATH}:" in
  *:"$INSTALL_DIR":*) ;;
  *) echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.bashrc"
     echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.zshrc" 2>/dev/null || true
     echo "Added $INSTALL_DIR to PATH in ~/.bashrc / ~/.zshrc" ;;
esac

echo ""
echo "Velocity installed successfully!"
echo "  Binary: $INSTALL_DIR/$BINARY"
echo "  Version: $TAG"
echo ""
echo "Restart your terminal or run: export PATH=\"\$PATH:$INSTALL_DIR\""
echo "Then run: velo --help"
