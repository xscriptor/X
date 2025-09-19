#!/bin/bash

set -e

# Detect package manager
if command -v pacman &>/dev/null; then
    PKG="pacman"
elif command -v apt &>/dev/null; then
    PKG="apt"
elif command -v dnf &>/dev/null; then
    PKG="dnf"
else
    echo "No compatible package manager found (dnf, apt, pacman)"
    exit 1
fi

echo "Using package manager: $PKG"

# Update & install depending on package manager
if [ "$PKG" = "pacman" ]; then
    sudo pacman -Syu --noconfirm
    sudo pacman -S --needed --noconfirm \
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

elif [ "$PKG" = "apt" ]; then
    sudo apt update && sudo apt upgrade -y
    sudo apt install -y \
        qemu-kvm \
        virt-manager \
        virt-viewer \
        dnsmasq \
        bridge-utils \
        libvirt-daemon-system \
        libvirt-clients \
        ovmf \
        iptables

elif [ "$PKG" = "dnf" ]; then
    sudo dnf upgrade -y
    sudo dnf install -y \
        @virtualization \
        qemu-kvm \
        virt-manager \
        virt-viewer \
        dnsmasq \
        bridge-utils \
        libvirt \
        edk2-ovmf \
        iptables
fi

# Enable and start libvirtd service
sudo systemctl enable --now libvirtd.service

# Add user to the libvirt group
sudo usermod -aG libvirt $USER

# Verify the status of the service
systemctl status libvirtd.service

echo "Finished. Restart to apply..."
