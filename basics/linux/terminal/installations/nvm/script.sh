#!/usr/bin/env bash
set -e

echo "Detecting Linux distribution..."

# --- Detect distro ---
if command -v pacman >/dev/null 2>&1; then
    DISTRO="arch"
elif command -v apt >/dev/null 2>&1; then
    DISTRO="ubuntu"
elif command -v dnf >/dev/null 2>&1; then
    DISTRO="fedora"
else
    echo "Unsupported distribution. Exiting."
    exit 1
fi

echo "Detected distro: $DISTRO"
echo

# --- Install base dependencies according to distro ---
case "$DISTRO" in
  arch)
    echo "Installing base packages for Arch..."
    sudo pacman -Syu --noconfirm
    sudo pacman -S --needed --noconfirm git curl wget base-devel ca-certificates lsb-release gnupg || true
    ;;
  ubuntu)
    echo "Installing base packages for Ubuntu/Debian..."
    sudo apt update -y && sudo apt upgrade -y
    sudo apt install -y git curl wget build-essential ca-certificates lsb-release gnupg || true
    ;;
  fedora)
    echo "Installing base packages for Fedora..."
    sudo dnf upgrade -y
    sudo dnf install -y git curl wget gcc-c++ make ca-certificates redhat-lsb-core gnupg2 || true
    ;;
esac

echo
echo "Base dependencies installed successfully."
echo

# --- Install NVM ---
if [ ! -d "$HOME/.nvm" ]; then
  echo "Installing NVM..."
  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash
else
  echo "NVM already installed."
fi

# --- Load NVM ---
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

echo
echo "Installing Node.js LTS..."
nvm install --lts
nvm use --lts

echo
echo "Node version:"
node -v
echo "NPM version:"
npm -v

# --- Update npm ---
echo
echo "Updating npm..."
npm install -g npm@latest

# --- Install TypeScript toolchain ---
echo
echo "Installing TypeScript toolchain..."
npm install -g typescript ts-node @types/node

# --- Install yarn + pnpm ---
echo
echo "Installing yarn + pnpm..."
npm install -g yarn pnpm

echo
echo "Done."
echo "TypeScript, ts-node, yarn, pnpm are now available globally (NVM global-local)."
