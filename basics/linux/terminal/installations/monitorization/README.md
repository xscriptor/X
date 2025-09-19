# Monitorization

---

## Personal monitorization tools:

- To Install the recomended personal selection:
- 1. Just click on [moninstall.sh](./moninstall.sh) download the script
- 2. Run 
```bash
chmod +x moninstall.sh
./moninstall.sh
```
or copy the script:

<details>
<summary>script</summary>

```bash

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

else
    echo "The sistem dosnt support pacman or apt..."
fi


```

</details>

---

## Other monitors:

### Full-System Monitors (Overview of CPU, RAM, I/O, etc.)

* htop	Interactive process viewer, more advanced than top.
* glances	Cross-platform system monitor with real-time info on CPU, RAM, disk, etc.
* atop	Advanced monitor with historical logging of system resource usage.
* btop	Modern and colorful TUI monitor for CPU, memory, disks, and processes.
* nmon	Comprehensive performance monitor for CPU, memory, disks, network, etc

---

### Network Monitors

* iftop	Shows bandwidth usage between local and remote hosts.
* iptraf	Real-time IP LAN monitor, shows traffic per connection.
* nethogs	Network monitor that groups usage by process.



