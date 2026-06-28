#!/usr/bin/env sh
# dkp installer — downloads the appropriate pre-built binary from GitHub Releases
set -eu

REPO="dkp-standard/cli"
BIN="dkp"

# Determine install prefix
if [ "$(id -u)" = "0" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="${HOME}/.local/bin"
fi

# Resolve version
if [ -z "${DKP_VERSION:-}" ]; then
    DKP_VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' | sed 's/.*"tag_name": *"\(.*\)".*/\1/')
fi

# Detect OS
OS=$(uname -s)
case "${OS}" in
    Linux)  OS_NAME="linux" ;;
    Darwin) OS_NAME="apple-darwin" ;;
    *)
        echo "Unsupported OS: ${OS}" >&2
        exit 1
        ;;
esac

# Detect arch
ARCH=$(uname -m)
case "${ARCH}" in
    x86_64)  ARCH_NAME="x86_64" ;;
    aarch64|arm64) ARCH_NAME="aarch64" ;;
    *)
        echo "Unsupported architecture: ${ARCH}" >&2
        exit 1
        ;;
esac

# Build artifact name
if [ "${OS_NAME}" = "apple-darwin" ]; then
    ARTIFACT="dkp-${DKP_VERSION}-universal-apple-darwin"
elif [ "${ARCH_NAME}" = "aarch64" ]; then
    ARTIFACT="dkp-${DKP_VERSION}-aarch64-unknown-linux-musl"
else
    ARTIFACT="dkp-${DKP_VERSION}-x86_64-unknown-linux-musl"
fi

URL="https://github.com/${REPO}/releases/download/${DKP_VERSION}/${ARTIFACT}.tar.gz"

echo "Downloading ${BIN} ${DKP_VERSION} for ${ARCH_NAME}-${OS_NAME}..."
curl --proto '=https' --tlsv1.2 -fLo /tmp/dkp.tar.gz "${URL}"

echo "Extracting..."
tar -C /tmp -xzf /tmp/dkp.tar.gz "${ARTIFACT}/${BIN}"

echo "Installing to ${INSTALL_DIR}/${BIN}..."
mkdir -p "${INSTALL_DIR}"
mv "/tmp/${ARTIFACT}/${BIN}" "${INSTALL_DIR}/${BIN}"
chmod +x "${INSTALL_DIR}/${BIN}"
rm -rf /tmp/dkp.tar.gz "/tmp/${ARTIFACT}"

echo "Installed ${BIN} ${DKP_VERSION} to ${INSTALL_DIR}/${BIN}"

# Warn if install dir is not on PATH
case ":${PATH}:" in
    *":${INSTALL_DIR}:"*) ;;
    *)
        echo ""
        echo "NOTE: ${INSTALL_DIR} is not on your PATH."
        echo "Add the following to your shell profile:"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
        ;;
esac
