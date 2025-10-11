#!/bin/bash

set -e

# Detectar gestor de paquetes
if command -v apt >/dev/null 2>&1; then
    PKG_MANAGER="apt"
    UPDATE_CMD="sudo apt update"
    INSTALL_CMD="sudo apt install -y"
    BASE_PACKAGES=(
        gnome-shell-extensions
        gnome-shell-extension-prefs
        chrome-gnome-shell
        curl
        jq
        unzip
        gnome-tweaks
        dconf-editor
        flatpak
        git
        make
    )
elif command -v pacman >/dev/null 2>&1; then
    PKG_MANAGER="pacman"
    UPDATE_CMD="sudo pacman -Syu --noconfirm"
    INSTALL_CMD="sudo pacman -S --noconfirm"
    BASE_PACKAGES=(
        gnome-shell-extensions
        gnome-shell-extension-apps
        chrome-gnome-shell
        curl
        jq
        unzip
        gnome-tweaks
        dconf-editor
        flatpak
        git
        make
    )
elif command -v dnf >/dev/null 2>&1; then
    PKG_MANAGER="dnf"
    UPDATE_CMD="sudo dnf check-update || true"
    INSTALL_CMD="sudo dnf install -y"
    BASE_PACKAGES=(
        gnome-extensions-app
        chrome-gnome-shell
        curl
        jq
        unzip
        gnome-tweaks
        dconf-editor
        flatpak
        git
        make
    )
else
    echo "No compatible package manager found (apt, pacman, dnf)."
    exit 1
fi

echo "Using $PKG_MANAGER as package manager"

echo "Updating system..."
eval "$UPDATE_CMD"

echo "Installing base packages..."
$INSTALL_CMD "${BASE_PACKAGES[@]}"

echo "Adding Flathub repository if not already added..."
sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

echo "Installing Extension Manager via Flatpak..."
flatpak install -y flathub com.mattjakeman.ExtensionManager

EXTENSIONS_DIR="$HOME/.local/share/gnome-shell/extensions"
mkdir -p "$EXTENSIONS_DIR"

# Special case: Dash to Dock GitHub
install_dash_to_dock() {
  echo "Installing Dash to Dock from GitHub..."
  rm -rf "$EXTENSIONS_DIR/dash-to-dock@micxgx.gmail.com"
  git clone https://github.com/micheleg/dash-to-dock.git
  cd dash-to-dock
  make install
  cd ..
  rm -rf dash-to-dock
  gnome-extensions enable dash-to-dock@micxgx.gmail.com || echo "Failed to enable Dash to Dock"
}

# Generic GNOME Shell extension installer
install_gnome_extension() {
  EXTENSION_ID=$1
  EXTENSION_NAME=$2

  echo "Installing $EXTENSION_NAME (ID $EXTENSION_ID)..."

  UUID=$(curl -s "https://extensions.gnome.org/extension-info/?pk=$EXTENSION_ID" | jq -r .uuid)
  SHELL_VERSION=$(gnome-shell --version | grep -oP '\d+\.\d+' | head -1)
  VERSION_TAG=$(curl -s "https://extensions.gnome.org/extension-info/?pk=$EXTENSION_ID" | jq -r '.shell_version_map["'"$SHELL_VERSION"'"] // .shell_version_map | keys[0]')
  DOWNLOAD_URL="https://extensions.gnome.org/download-extension/$UUID.shell-extension.zip?version_tag=$VERSION_TAG"

  DEST="$EXTENSIONS_DIR/$UUID"
  mkdir -p "$DEST"
  curl -L "$DOWNLOAD_URL" -o "$DEST/extension.zip"

  if unzip -t "$DEST/extension.zip" >/dev/null 2>&1; then
    unzip -o "$DEST/extension.zip" -d "$DEST"
    gnome-extensions enable "$UUID" || echo "Manual activation may be required for $UUID"
  else
    echo "Warning: Failed to install $EXTENSION_NAME. Invalid or corrupted download."
    rm "$DEST/extension.zip"
  fi
}

# First: install Dash to Dock manually
install_dash_to_dock

# Other extensions (excluding Dash to Dock)
EXTENSIONS=(
  "3193 Blur_My_Shell"
  "19 User_Themes"
  "3843 Just_Perfection"
  "1460 Vitals"
  "779 Clipboard_Indicator"
  "517 Caffeine"
  "2236 Night_Theme_Switcher"
  "1319 GSConnect"
  "358 Media_Controls"
  "750 OpenWeather"
  "1401 Bluetooth_Quick_Connect"
)

for item in "${EXTENSIONS[@]}"; do
  install_gnome_extension $item
done

echo "All extensions processed."

echo "Attempting to reload GNOME Shell (only works on X11)..."
busctl --user call org.gnome.Shell /org/gnome/Shell org.gnome.Shell Eval s 'global.reexec_self()' || true

echo "Done. You can manage your extensions via 'Extension Manager' or 'gnome-extensions-app'."
