
# Only Arch

sudo pacman -Syu --needed \
  qemu-full libvirt virt-manager virt-viewer edk2-ovmf dnsmasq swtpm guestfs-tools libosinfo \
  bridge-utils vde2 openbsd-netcat ebtables iptables

sudo systemctl enable --now libvirtd.service
sudo systemctl enable --now virtlogd.socket
sudo systemctl enable --now virtlockd.socket

sudo usermod -a -G libvirt,kvm $(whoami)
sudo virsh net-autostart default
sudo virsh net-start default
