#!/usr/bin/env bash
set -euo pipefail

echo "[XOs] Installing QEMU + libvirt + virt-manager stack…"

sudo pacman -Syu --needed \
  qemu-full libvirt virt-manager virt-viewer edk2-ovmf dnsmasq swtpm guestfs-tools libosinfo \
  bridge-utils vde2 openbsd-netcat ebtables iptables-nft

echo "[XOs] Enabling services..."
sudo systemctl enable --now libvirtd.service
sudo systemctl enable --now virtlogd.socket virtlockd.socket

echo "[XOs] Adding user to libvirt/kvm groups..."
sudo usermod -aG libvirt,kvm "$(whoami)"

echo "[XOs] Restarting libvirt service..."
sudo systemctl restart libvirtd.service

echo "[XOs] Defining and starting default network..."
sudo virsh net-define /usr/share/libvirt/networks/default.xml 2>/dev/null || true
sudo virsh net-autostart default || true
sudo virsh net-start default || true

echo "[XOs] Virtualization stack installed."
echo "[XOs] Please log out or reboot to apply group changes."
echo "[XOs] After reboot, run virt-manager to start using QEMU/KVM."