# basic desktop customization

## Here's a full script to install GNOME Shell extensions and related tools on a Debian/Ubuntu-based system. It includes the installation of:

* Installs base packages for GNOME Shell extension support
* Installs Extension Manager (GUI)
* Installs and enables the following extensions:
    * Dash to Dock
    * Blur My Shell
    * User Themes
    * Just Perfection
    * Vitals
    * Clipboard Indicator
    * Caffeine
    * Night Theme Switcher
    * GSConnect
    * Media Controls
    * OpenWeather
    * Bluetooth Quick Connect
* Installs additional tools: gnome-tweaks, dconf-editor

### Note: If any extensions fail to install automatically, you can find and install them manually using the Extension Manager.


## Instructions:
* 1. You can copy and paste the following into your terminal:


<details>
<summary>basic-desktop-instalation.sh</summary>

```bash

#!/bin/bash

set -e

echo "Updating system and installing base packages..."
sudo apt update
sudo apt install -y \
  gnome-shell-extensions \
  gnome-shell-extension-prefs \
  chrome-gnome-shell \
  curl \
  jq \
  unzip \
  gnome-tweaks \
  dconf-editor \
  flatpak \
  git \
  make

echo "Adding Flathub repository if not already added..."
sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

echo "Installing Extension Manager via Flatpak..."
flatpak install -y flathub com.mattjakeman.ExtensionManager

EXTENSIONS_DIR="$HOME/.local/share/gnome-shell/extensions"
mkdir -p "$EXTENSIONS_DIR"

# Special case: Dash to Dock from GitHub
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
#!/bin/bash

# Update repos
sudo apt update

# Install QEMU, libvirt, virt-manager y otras and dependencies
sudo apt install \
  qemu \
  virt-manager \
  virt-viewer \
  dnsmasq \
  vde2 \
  bridge-utils \
  openbsd-netcat \
  libvirt \
  edk2-ovmf \
  iptables-nft

# Enable e and start libvirtd service
sudo systemctl enable --now libvirtd.service

# add user to the libvirt group
sudo usermod -aG libvirt $USER

# verify the status of the service
systemctl status libvirtd.service

echo "Finished. Restart to apply."

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



```

</details>

* 2. You can download the script:

[basic-desktop-instalation.sh](./bdesktopi.sh)

and run:

```bash

chmod +x bdesktopi.sh
./bdesktopi.sh

```

## Requirements

- Debian-based system
- GNOME Shell
- Flatpak enabled