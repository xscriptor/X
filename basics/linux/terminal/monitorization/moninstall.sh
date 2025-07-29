#!/bin/bash

echo "Installing system monitors..."
sudo apt update

# Install all selected tools
sudo apt install -y btop iotop nethogs powertop nvtop gnome-terminal

echo "Installation complete."

# Launch each monitor in its own terminal window
echo "Launching monitors in separate terminals..."

gnome-terminal -- bash -c "echo 'Launching btop (system overview)...'; btop"
gnome-terminal -- bash -c "echo 'Launching iotop (disk I/O)...'; sudo iotop"
gnome-terminal -- bash -c "echo 'Launching nethogs (network usage)...'; sudo nethogs"
gnome-terminal -- bash -c "echo 'Launching powertop (power analysis)...'; sudo powertop"
gnome-terminal -- bash -c "echo 'Launching nvtop (GPU monitor)...'; nvtop"

echo "All monitors launched."
