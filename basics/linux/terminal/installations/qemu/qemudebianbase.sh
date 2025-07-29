#!/bin/bash

# Update repositories
sudo apt update

# Install virtualization packages
sudo apt install -y \
  qemu \
  virt-manager \
  virt-viewer \
  dnsmasq \
  vde2 \
  bridge-utils \
  openbsd-netcat \
  libvirt-daemon-system \
  libvirt-clients \
  ovmf \
  iptables-nft

# Enable and start libvirtd
sudo systemctl enable --now libvirtd.service

# Add user to libvirt and kvm groups
sudo usermod -aG libvirt $USER
sudo usermod -aG kvm $USER

# Check service status
systemctl status libvirtd.service

echo "Installation complete. Please restart your system to apply group changes."