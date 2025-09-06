use sysinfo::{System, Cpu, Disk};
use colored::*;
use std::env;
use std::process::Command;
use std::fs;
use std::path::Path;

pub struct SystemInfo {
    pub os_name: String,
    pub kernel_version: String,
    pub hostname: String,
    pub uptime: String,
    pub cpu_info: String,
    pub memory_info: String,
    pub disk_info: String,
    pub shell: String,
    pub terminal: String,
    pub gpu_info: String,
    pub packages: String,
    pub color_scheme: String,
}

impl SystemInfo {
    pub fn new(sys: &System) -> Self {
        SystemInfo {
            os_name: Self::get_os_name(sys),
            kernel_version: Self::get_kernel_version(sys),
            hostname: Self::get_hostname(sys),
            uptime: Self::get_uptime(sys),
            cpu_info: Self::get_cpu_info(sys),
            memory_info: Self::get_memory_info(sys),
            disk_info: Self::get_disk_info(),
            shell: Self::get_shell(),
            terminal: Self::get_terminal(),
            gpu_info: Self::get_gpu_info(),
            packages: Self::get_packages(),
            color_scheme: Self::get_color_scheme(),
        }
    }

    fn get_os_name(sys: &System) -> String {
        let os = System::name().unwrap_or_else(|| "Unknown".to_string());
        let version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        format!("{} {}", os, version)
    }

    fn get_kernel_version(_sys: &System) -> String {
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    }

    fn get_hostname(_sys: &System) -> String {
        System::host_name().unwrap_or_else(|| "Unknown".to_string())
    }

    fn get_uptime(_sys: &System) -> String {
        let uptime_seconds = System::uptime();
        let days = uptime_seconds / 86400;
        let hours = (uptime_seconds % 86400) / 3600;
        let minutes = (uptime_seconds % 3600) / 60;
        
        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }

    fn get_cpu_info(sys: &System) -> String {
        if let Some(cpu) = sys.cpus().first() {
            let brand = cpu.brand();
            let cores = sys.cpus().len();
            format!("{} ({} cores)", brand, cores)
        } else {
            "Unknown".to_string()
        }
    }

    fn get_memory_info(sys: &System) -> String {
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let total_gb = total_memory as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = used_memory as f64 / 1024.0 / 1024.0 / 1024.0;
        let percentage = (used_memory as f64 / total_memory as f64) * 100.0;
        
        format!("{:.1}GB / {:.1}GB ({:.1}%)", used_gb, total_gb, percentage)
    }

    fn get_disk_info() -> String {
        let disks = sysinfo::Disks::new_with_refreshed_list();
        if let Some(main_disk) = disks.first() {
            let total_space = main_disk.total_space();
            let available_space = main_disk.available_space();
            let used_space = total_space - available_space;
            
            let total_gb = total_space as f64 / 1024.0 / 1024.0 / 1024.0;
            let used_gb = used_space as f64 / 1024.0 / 1024.0 / 1024.0;
            let percentage = (used_space as f64 / total_space as f64) * 100.0;
            
            format!("{:.1}GB / {:.1}GB ({:.1}%)", used_gb, total_gb, percentage)
        } else {
            "Unknown".to_string()
        }
    }

    fn get_shell() -> String {
        env::var("SHELL")
            .or_else(|_| env::var("ComSpec"))
            .map(|shell| {
                shell.split(['/', '\\']).last().unwrap_or(&shell).to_string()
            })
            .unwrap_or_else(|_| "Unknown".to_string())
    }

    fn get_terminal() -> String {
        env::var("TERM")
            .or_else(|_| env::var("TERMINAL"))
            .or_else(|_| env::var("TERM_PROGRAM"))
            .unwrap_or_else(|_| "Unknown".to_string())
    }

    fn get_gpu_info() -> String {
        #[cfg(windows)]
        {
            // Try PowerShell first as it's more reliable
            if let Ok(output) = Command::new("powershell")
                .args(["-Command", "Get-WmiObject -Class Win32_VideoController | Select-Object -ExpandProperty Name | Select-Object -First 1"])
                .output()
            {
                let gpu_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !gpu_name.is_empty() && gpu_name != "Unknown" {
                    return gpu_name;
                }
            }
            
            // Fallback to wmic
            if let Ok(output) = Command::new("wmic")
                .args(["path", "win32_VideoController", "get", "name"])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = output_str.lines().collect();
                if lines.len() > 1 {
                    let gpu_name = lines[1].trim();
                    if !gpu_name.is_empty() {
                        return gpu_name.to_string();
                    }
                }
            }
        }
        
        #[cfg(unix)]
        {
            if let Ok(output) = Command::new("lspci").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("VGA") || line.contains("3D") {
                        if let Some(gpu_part) = line.split(':').nth(2) {
                            return gpu_part.trim().to_string();
                        }
                    }
                }
            }
        }
        
        "Unknown".to_string()
    }

    fn get_packages() -> String {
        #[cfg(windows)]
        {
            // Try to count installed programs via registry or chocolatey
            if let Ok(output) = Command::new("powershell")
                .args(["-Command", "(Get-ItemProperty HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*).DisplayName.Count"])
                .output()
            {
                let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !count_str.is_empty() && count_str != "0" {
                    return format!("{} (programs)", count_str);
                }
            }
            
            // Try chocolatey as fallback
            if let Ok(output) = Command::new("choco").args(["list", "--local-only"]).output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let count = output_str.lines().filter(|line| !line.is_empty() && !line.contains("packages installed")).count();
                if count > 0 {
                    return format!("{} (choco)", count);
                }
            }
        }
        
        #[cfg(unix)]
        {
            // Try different package managers
            let package_managers = [
                ("dpkg", &["-l"] as &[&str]),
                ("rpm", &["-qa"]),
                ("pacman", &["-Q"]),
                ("apk", &["list", "--installed"]),
                ("brew", &["list"]),
            ];
            
            for (manager, args) in package_managers {
                if let Ok(output) = Command::new(manager).args(args).output() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    let count = output_str.lines().filter(|line| !line.trim().is_empty()).count();
                    if count > 0 {
                        return format!("{} ({})", count, manager);
                    }
                }
            }
        }
        
        "Unknown".to_string()
    }

    fn get_color_scheme() -> String {
        // Try to detect terminal color scheme
        if let Ok(colorterm) = env::var("COLORTERM") {
            if colorterm.contains("truecolor") || colorterm.contains("24bit") {
                return "24-bit (truecolor)".to_string();
            }
        }
        
        if let Ok(term) = env::var("TERM") {
            if term.contains("256color") {
                return "256 colors".to_string();
            } else if term.contains("color") {
                return "16 colors".to_string();
            }
        }
        
        // Check Windows Terminal or PowerShell
        if env::var("WT_SESSION").is_ok() {
            return "Windows Terminal (24-bit)".to_string();
        }
        
        if env::var("PSModulePath").is_ok() {
            return "PowerShell (16 colors)".to_string();
        }
        
        "Unknown".to_string()
    }

    pub fn format_info(&self) -> Vec<String> {
        vec![
            format!("{}{}", "OS: ".bright_cyan().bold(), self.os_name.white()),
            format!("{}{}", "Kernel: ".bright_cyan().bold(), self.kernel_version.white()),
            format!("{}{}", "Host: ".bright_cyan().bold(), self.hostname.white()),
            format!("{}{}", "Uptime: ".bright_cyan().bold(), self.uptime.white()),
            format!("{}{}", "Shell: ".bright_cyan().bold(), self.shell.white()),
            format!("{}{}", "Terminal: ".bright_cyan().bold(), self.terminal.white()),
            format!("{}{}", "CPU: ".bright_cyan().bold(), self.cpu_info.white()),
            format!("{}{}", "GPU: ".bright_cyan().bold(), self.gpu_info.white()),
            format!("{}{}", "Memory: ".bright_cyan().bold(), self.memory_info.white()),
            format!("{}{}", "Disk: ".bright_cyan().bold(), self.disk_info.white()),
            format!("{}{}", "Packages: ".bright_cyan().bold(), self.packages.white()),
            format!("{}{}", "Colors: ".bright_cyan().bold(), self.color_scheme.white()),
        ]
    }

    pub fn display_color_palette() -> Vec<String> {
        let mut palette = Vec::new();
        
        // Add empty line for spacing
        palette.push("".to_string());
        
        // Basic colors (normal)
        let basic_colors = [
            ("black", "███".black()),
            ("red", "███".red()),
            ("green", "███".green()),
            ("yellow", "███".yellow()),
            ("blue", "███".blue()),
            ("magenta", "███".magenta()),
            ("cyan", "███".cyan()),
            ("white", "███".white()),
        ];
        
        let mut basic_line = String::new();
        for (_, color_block) in &basic_colors {
            basic_line.push_str(&format!("{}", color_block));
        }
        palette.push(basic_line);
        
        // Bright colors
        let bright_colors = [
            ("bright_black", "███".bright_black()),
            ("bright_red", "███".bright_red()),
            ("bright_green", "███".bright_green()),
            ("bright_yellow", "███".bright_yellow()),
            ("bright_blue", "███".bright_blue()),
            ("bright_magenta", "███".bright_magenta()),
            ("bright_cyan", "███".bright_cyan()),
            ("bright_white", "███".bright_white()),
        ];
        
        let mut bright_line = String::new();
        for (_, color_block) in &bright_colors {
            bright_line.push_str(&format!("{}", color_block));
        }
        palette.push(bright_line);
        
        palette
    }
}