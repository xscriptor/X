# Monitorization

Here you will find a list of T-UI resources to look whats is happening on your system


* htop(recomended):

```bash
sudo apt install htop -y
```

* nvtop(recomended to check the gpu):

```bash
sudo apt install nvtop -y
```


### Full-System Monitors (Overview of CPU, RAM, I/O, etc.)

* htop	Interactive process viewer, more advanced than top.
* glances	Cross-platform system monitor with real-time info on CPU, RAM, disk, etc.
* atop	Advanced monitor with historical logging of system resource usage.
* btop	Modern and colorful TUI monitor for CPU, memory, disks, and processes.
* nmon	Comprehensive performance monitor for CPU, memory, disks, network, etc

---

### I/O and Disk Usage Monitors

* iotop	Shows real-time disk I/O usage by processes (root needed).
* dstat	Combines vmstat, iostat, netstat. Extensible and scriptable.
* iostat	From sysstat, shows CPU and I/O statistics.

---

### Network Monitors

* iftop	Shows bandwidth usage between local and remote hosts.
* iptraf	Real-time IP LAN monitor, shows traffic per connection.
* nethogs	Network monitor that groups usage by process.


### Power and Frequency Monitors
* powertop	Monitor power usage and power-saving suggestions.
* cpufrequtils	Shows and sets CPU frequency scaling governors.

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


```

</details>