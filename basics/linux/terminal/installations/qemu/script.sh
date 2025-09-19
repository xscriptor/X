#!/bin/bash

# Update repos
sudo pacman -Syu --noconfirm

# Install QEMU, libvirt, virt-manager y otras and dependencies
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

# Enable e and start libvirtd service
sudo systemctl enable --now libvirtd.service

# add user to the libvirt group
sudo usermod -aG libvirt $USER

# verify the status of the service
systemctl status libvirtd.service

echo "Finished. Restart to apply."
