#!/bin/bash

echo "Installing system monitors..."
if command -v dpkg &> /dev/null; then
    echo "apt updating..."
    sudo apt update
    echo "Installing monirotization tools..."
    sudo apt install -y btop nvtop nethogs
    echo "Installation complete."
    echo "To run just use btop and nvtop from terminal."

elif command -v pacman &> /dev/null; then
    echo "pacman updating"
    sudo pacman -Syu
    echo "Installing monirotization tools..."
    sudo pacman -S --noconfirm btop nvtop nethogs
    echo "Installation complete."
    echo "To run just use btop and nvtop from terminal."

    elif command -v dnf &> /dev/null; then
    echo "dnf updating"
    sudo pacman -Syu
    echo "Installing monirotization tools..."
    sudo dnf install -y btop nvtop nethogs
    echo "Installation complete."
    echo "To run just use btop and nvtop from terminal."

else
    echo "The sistem dosnt support pacman, apt or dnf..."
fi