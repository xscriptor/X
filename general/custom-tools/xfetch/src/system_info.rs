use sysinfo::System;
use colored::*;
use std::env;
use std::process::Command;
use chrono::Local;
use crate::SectionFilter;


#[cfg(windows)]
use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
#[cfg(windows)]
use std::mem;

// Custom color palette
const WARM_YELLOW: (u8, u8, u8) = (252, 229, 102); // #FCE566
const CORAL: (u8, u8, u8) = (252, 97, 141);         // #FC618D
const MINT: (u8, u8, u8) = (123, 216, 143);         // #7BD88F
const SKY_BLUE: (u8, u8, u8) = (90, 212, 230);      // #5AD4E6
const PURPLE: (u8, u8, u8) = (148, 138, 227);       // #948AE3

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
    // New extended fields
    pub windows_build: String,
    pub architecture: String,
    pub manufacturer: String,
    pub model: String,
    pub machine_uuid: String,
    pub terminal_font: String,
    pub timezone: String,
    pub cpu_speed: String,
    pub cpu_temp: String,
    pub cpu_load: String,
    pub gpu_temp: String,
    pub gpu_vram: String,
    pub monitors: String,
    pub battery: String,
    pub network_info: String,
    pub powershell_version: String,
    pub dotnet_version: String,
    pub desktop_environment: String,
    pub system_language: String,
    pub active_processes: String,
    pub top_processes: String,
    pub disk_speed: String,
    pub network_usage: String,
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
            // Initialize new extended fields
            windows_build: Self::get_windows_build(),
            architecture: Self::get_architecture(),
            manufacturer: Self::get_manufacturer(),
            model: Self::get_model(),
            machine_uuid: Self::get_machine_uuid(),
            terminal_font: Self::get_terminal_font(),
            timezone: Self::get_timezone(),
            cpu_speed: Self::get_cpu_speed(sys),
            cpu_temp: Self::get_cpu_temp(),
            cpu_load: Self::get_cpu_load(sys),
            gpu_temp: Self::get_gpu_temp(),
            gpu_vram: Self::get_gpu_vram(),
            monitors: Self::get_monitors(),
            battery: Self::get_battery(),
            network_info: Self::get_network_info(),
            powershell_version: Self::get_powershell_version(),
            dotnet_version: Self::get_dotnet_version(),
            desktop_environment: Self::get_desktop_environment(),
            system_language: Self::get_system_language(),
            active_processes: Self::get_active_processes(sys),
            top_processes: Self::get_top_processes(sys),
            disk_speed: Self::get_disk_speed(),
            network_usage: Self::get_network_usage(),
        }
    }

    pub fn new_filtered(sys: &System, filter: &SectionFilter) -> Self {
        SystemInfo {
            // Always collect basic OS info (needed for all modes)
            os_name: Self::get_os_name(sys),
            kernel_version: if filter.show_os { Self::get_kernel_version(sys) } else { "Unknown".to_string() },
            hostname: Self::get_hostname(sys),
            uptime: if filter.show_os { Self::get_uptime(sys) } else { "Unknown".to_string() },
            shell: Self::get_shell(),
            terminal: Self::get_terminal(),
            
            // System info - only collect if needed
            packages: if filter.show_system || filter.summary_mode { Self::get_packages() } else { "Unknown".to_string() },
            color_scheme: if filter.show_system || filter.summary_mode { Self::get_color_scheme() } else { "Unknown".to_string() },
            powershell_version: if filter.show_system { Self::get_powershell_version() } else { "Unknown".to_string() },
            dotnet_version: if filter.show_system { Self::get_dotnet_version() } else { "Unknown".to_string() },
            desktop_environment: if filter.show_system { Self::get_desktop_environment() } else { "Unknown".to_string() },
            system_language: if filter.show_system { Self::get_system_language() } else { "Unknown".to_string() },
            
            // Hardware info - only collect if needed
            cpu_info: if filter.show_hardware || filter.summary_mode { Self::get_cpu_info(sys) } else { "Unknown".to_string() },
            memory_info: if filter.show_hardware || filter.summary_mode { Self::get_memory_info(sys) } else { "Unknown".to_string() },
            disk_info: if filter.show_hardware || filter.summary_mode { Self::get_disk_info() } else { "Unknown".to_string() },
            gpu_info: if filter.show_hardware || filter.summary_mode { Self::get_gpu_info() } else { "Unknown".to_string() },
            cpu_speed: if filter.show_hardware { Self::get_cpu_speed(sys) } else { "Unknown".to_string() },
            cpu_temp: if filter.show_hardware { Self::get_cpu_temp() } else { "N/A".to_string() },
            cpu_load: if filter.show_hardware { Self::get_cpu_load(sys) } else { "Unknown".to_string() },
            gpu_temp: if filter.show_hardware { Self::get_gpu_temp() } else { "N/A".to_string() },
            gpu_vram: if filter.show_hardware { Self::get_gpu_vram() } else { "Unknown".to_string() },
            monitors: if filter.show_hardware { Self::get_monitors() } else { "Unknown".to_string() },
            battery: if filter.show_hardware { Self::get_battery() } else { "N/A".to_string() },
            
            // Network info - only collect if needed
            network_info: if filter.show_network { Self::get_network_info() } else { "Unknown".to_string() },
            network_usage: if filter.show_network { Self::get_network_usage() } else { "N/A".to_string() },
            
            // Statistics - only collect if needed
            active_processes: if filter.show_stats { Self::get_active_processes(sys) } else { "Unknown".to_string() },
            top_processes: if filter.show_stats { Self::get_top_processes(sys) } else { "Unknown".to_string() },
            disk_speed: if filter.show_stats { Self::get_disk_speed() } else { "N/A".to_string() },
            
            // Extended OS fields - only collect if OS section is needed
            windows_build: if filter.show_os { Self::get_windows_build() } else { "Unknown".to_string() },
            architecture: if filter.show_os { Self::get_architecture() } else { "Unknown".to_string() },
            manufacturer: if filter.show_system { Self::get_manufacturer() } else { "Unknown".to_string() },
            model: if filter.show_system { Self::get_model() } else { "Unknown".to_string() },
            machine_uuid: if filter.show_system { Self::get_machine_uuid() } else { "Unknown".to_string() },
            terminal_font: if filter.show_os { Self::get_terminal_font() } else { "Unknown".to_string() },
            timezone: if filter.show_os { Self::get_timezone() } else { "Unknown".to_string() },
        }
    }

    fn get_os_name(_sys: &System) -> String {
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
                .args(["-Command", r"(Get-ItemProperty HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*).DisplayName.Count"])
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
        // Try to detect color scheme from environment variables or registry
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", r"Get-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize' -Name AppsUseLightTheme -ErrorAction SilentlyContinue | Select-Object -ExpandProperty AppsUseLightTheme"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let result = output_str.trim();
            match result {
                "0" => "Dark".to_string(),
                "1" => "Light".to_string(),
                _ => {
                    // Fallback: check for common environment variables
                    if env::var("TERM").unwrap_or_default().contains("256color") {
                        "24-bit (truecolor)".to_string()
                    } else {
                        "Unknown".to_string()
                    }
                }
            }
        } else {
            // Fallback for non-Windows or when PowerShell is not available
            if env::var("TERM").unwrap_or_default().contains("256color") {
                "24-bit (truecolor)".to_string()
            } else {
                "Unknown".to_string()
            }
        }
    }

    // New extended system information functions
    #[cfg(windows)]
    fn get_windows_build() -> String {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "Get-ComputerInfo | Select-Object WindowsProductName, WindowsBuildLabEx, WindowsInstallDateFromRegistry | Format-List"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = result.lines().collect();
            let mut build_info = Vec::new();
            
            for line in lines {
                if line.contains("WindowsBuildLabEx") {
                    if let Some(build) = line.split(':').nth(1) {
                        build_info.push(format!("Build {}", build.trim()));
                    }
                }
                if line.contains("WindowsInstallDateFromRegistry") {
                    if let Some(date) = line.split(':').nth(1) {
                        build_info.push(format!("Installed {}", date.trim()));
                    }
                }
            }
            
            if build_info.is_empty() {
                "Unknown".to_string()
            } else {
                build_info.join(", ")
            }
        } else {
            "Unknown".to_string()
        }
    }

    #[cfg(not(windows))]
    fn get_windows_build() -> String {
        "N/A (Not Windows)".to_string()
    }

    fn get_architecture() -> String {
        #[cfg(windows)]
        {
            unsafe {
                let mut sys_info: SYSTEM_INFO = mem::zeroed();
                GetSystemInfo(&mut sys_info);
                match sys_info.u.s().wProcessorArchitecture {
                    9 => "x64 (AMD64)".to_string(),
                    5 => "ARM".to_string(),
                    12 => "ARM64".to_string(),
                    0 => "x86".to_string(),
                    _ => "Unknown".to_string(),
                }
            }
        }
        #[cfg(not(windows))]
        {
            env::consts::ARCH.to_string()
        }
    }

    #[cfg(windows)]
    fn get_manufacturer() -> String {
        if let Ok(output) = Command::new("wmic")
            .args(["computersystem", "get", "manufacturer", "/value"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            for line in result.lines() {
                if line.starts_with("Manufacturer=") {
                    return line.replace("Manufacturer=", "").trim().to_string();
                }
            }
        }
        "Unknown".to_string()
    }

    #[cfg(not(windows))]
    fn get_manufacturer() -> String {
        if let Ok(output) = Command::new("dmidecode")
            .args(["-s", "system-manufacturer"])
            .output()
        {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "Unknown".to_string()
        }
    }

    #[cfg(windows)]
    fn get_model() -> String {
        if let Ok(output) = Command::new("wmic")
            .args(["computersystem", "get", "model", "/value"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            for line in result.lines() {
                if line.starts_with("Model=") {
                    return line.replace("Model=", "").trim().to_string();
                }
            }
        }
        "Unknown".to_string()
    }

    #[cfg(not(windows))]
    fn get_model() -> String {
        if let Ok(output) = Command::new("dmidecode")
            .args(["-s", "system-product-name"])
            .output()
        {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "Unknown".to_string()
        }
    }

    #[cfg(windows)]
    fn get_machine_uuid() -> String {
        if let Ok(output) = Command::new("wmic")
            .args(["csproduct", "get", "uuid", "/value"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            for line in result.lines() {
                if line.starts_with("UUID=") {
                    let uuid = line.replace("UUID=", "").trim().to_string();
                    if !uuid.is_empty() && uuid != "FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF" {
                        return format!("{}...{}", &uuid[..8], &uuid[uuid.len()-8..]);
                    }
                }
            }
        }
        "Unknown".to_string()
    }

    #[cfg(not(windows))]
    fn get_machine_uuid() -> String {
        if let Ok(output) = Command::new("dmidecode")
            .args(["-s", "system-uuid"])
            .output()
        {
            let uuid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !uuid.is_empty() {
                format!("{}...{}", &uuid[..8], &uuid[uuid.len()-8..])
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        }
    }

    fn get_terminal_font() -> String {
        #[cfg(windows)]
        {
            if let Ok(output) = Command::new("powershell")
                .args(["-Command", r"Get-ItemProperty -Path 'HKCU:\Console' -Name FaceName -ErrorAction SilentlyContinue | Select-Object -ExpandProperty FaceName"])
                .output()
            {
                let font = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !font.is_empty() && font != "0" {
                    return font;
                }
            }
            
            // Try Windows Terminal settings
            if let Ok(output) = Command::new("powershell")
                .args(["-Command", r"if (Test-Path '$env:LOCALAPPDATA\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json') { (Get-Content '$env:LOCALAPPDATA\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json' | ConvertFrom-Json).profiles.defaults.font.face }"])
                .output()
            {
                let font = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !font.is_empty() && font != "null" {
                    return font;
                }
            }
        }
        
        "Unknown".to_string()
    }

    fn get_timezone() -> String {
        let local_time = Local::now();
        format!("{}", local_time.format("%Z %z"))
    }

    fn get_cpu_speed(sys: &System) -> String {
        if let Some(cpu) = sys.cpus().first() {
            format!("{:.2} GHz", cpu.frequency() as f64 / 1000.0)
        } else {
            "Unknown".to_string()
        }
    }

    #[cfg(windows)]
    fn get_cpu_temp() -> String {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "Get-WmiObject -Namespace 'root/OpenHardwareMonitor' -Class Sensor | Where-Object { $_.SensorType -eq 'Temperature' -and $_.Name -like '*CPU*' } | Select-Object -First 1 -ExpandProperty Value"])
            .output()
        {
            let temp_output = String::from_utf8_lossy(&output.stdout);
            let temp = temp_output.trim();
            if !temp.is_empty() && temp != "" {
                return format!("{}°C", temp);
            }
        }
        
        // Alternative method using WMI
        if let Ok(output) = Command::new("wmic")
            .args([r"/namespace:\\root\wmi", "path", "MSAcpi_ThermalZoneTemperature", "get", "CurrentTemperature", "/value"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            for line in result.lines() {
                if line.starts_with("CurrentTemperature=") {
                    if let Ok(temp_raw) = line.replace("CurrentTemperature=", "").trim().parse::<f64>() {
                        let temp_celsius = (temp_raw / 10.0) - 273.15;
                        return format!("{:.1}°C", temp_celsius);
                    }
                }
            }
        }
        
        "N/A".to_string()
    }

    #[cfg(not(windows))]
    fn get_cpu_temp() -> String {
        if let Ok(output) = Command::new("sensors")
            .args(["-u"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            for line in result.lines() {
                if line.contains("temp1_input") {
                    if let Some(temp_str) = line.split_whitespace().nth(1) {
                        if let Ok(temp) = temp_str.parse::<f64>() {
                            return format!("{:.1}°C", temp);
                        }
                    }
                }
            }
        }
        "N/A".to_string()
    }

    fn get_cpu_load(sys: &System) -> String {
        let cpus = sys.cpus();
        if cpus.is_empty() {
            return "Unknown".to_string();
        }
        
        let total_usage: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
        let avg_usage = total_usage / cpus.len() as f32;
        
        // Show per-core usage for first 4 cores
        let mut core_usage = Vec::new();
        for (i, cpu) in cpus.iter().take(4).enumerate() {
            core_usage.push(format!("C{}: {:.1}%", i, cpu.cpu_usage()));
        }
        
        if cpus.len() > 4 {
            core_usage.push(format!("... ({} cores)", cpus.len()));
        }
        
        format!("Avg: {:.1}% [{}]", avg_usage, core_usage.join(", "))
    }

    #[cfg(windows)]
    fn get_gpu_temp() -> String {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "Get-WmiObject -Namespace 'root/OpenHardwareMonitor' -Class Sensor | Where-Object { $_.SensorType -eq 'Temperature' -and $_.Name -like '*GPU*' } | Select-Object -First 1 -ExpandProperty Value"])
            .output()
        {
            let temp_output = String::from_utf8_lossy(&output.stdout);
            let temp = temp_output.trim();
            if !temp.is_empty() && temp != "" {
                return format!("{}°C", temp);
            }
        }
        "N/A".to_string()
    }

    #[cfg(not(windows))]
    fn get_gpu_temp() -> String {
        if let Ok(output) = Command::new("nvidia-smi")
            .args(["--query-gpu=temperature.gpu", "--format=csv,noheader,nounits"])
            .output()
        {
            let temp_output = String::from_utf8_lossy(&output.stdout);
            let temp = temp_output.trim();
            if !temp.is_empty() {
                return format!("{}°C", temp);
            }
        }
        "N/A".to_string()
    }

    #[cfg(windows)]
    fn get_gpu_vram() -> String {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "Get-WmiObject -Class Win32_VideoController | Select-Object -First 1 -ExpandProperty AdapterRAM"])
            .output()
        {
            let vram_output = String::from_utf8_lossy(&output.stdout);
            let vram_bytes = vram_output.trim();
            if let Ok(vram) = vram_bytes.parse::<u64>() {
                let vram_gb = vram as f64 / (1024.0 * 1024.0 * 1024.0);
                return format!("{:.1} GB", vram_gb);
            }
        }
        "Unknown".to_string()
    }

    #[cfg(not(windows))]
    fn get_gpu_vram() -> String {
        if let Ok(output) = Command::new("nvidia-smi")
            .args(["--query-gpu=memory.total", "--format=csv,noheader,nounits"])
            .output()
        {
            let vram_output = String::from_utf8_lossy(&output.stdout);
            let vram_mb = vram_output.trim();
            if let Ok(vram) = vram_mb.parse::<f64>() {
                return format!("{:.1} GB", vram / 1024.0);
            }
        }
        "Unknown".to_string()
    }

    #[cfg(windows)]
    fn get_monitors() -> String {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "Get-WmiObject -Class Win32_DesktopMonitor | ForEach-Object { \"$($_.ScreenWidth)x$($_.ScreenHeight)\" } | Sort-Object -Unique"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            let monitors: Vec<&str> = result.lines().filter(|line| !line.trim().is_empty() && line.contains("x")).collect();
            if !monitors.is_empty() {
                return format!("{} monitor(s): {}", monitors.len(), monitors.join(", "));
            }
        }
        "Unknown".to_string()
    }

    #[cfg(not(windows))]
    fn get_monitors() -> String {
        if let Ok(output) = Command::new("xrandr")
            .args(["--query"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            let mut monitors = Vec::new();
            for line in result.lines() {
                if line.contains(" connected") {
                    monitors.push(line.split_whitespace().next().unwrap_or("Unknown"));
                }
            }
            if !monitors.is_empty() {
                return format!("{} monitor(s): {}", monitors.len(), monitors.join(", "));
            }
        }
        "Unknown".to_string()
    }

    #[cfg(windows)]
    fn get_battery() -> String {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "Get-WmiObject -Class Win32_Battery | Select-Object -First 1 EstimatedChargeRemaining, BatteryStatus, DesignCapacity, FullChargeCapacity"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = result.lines().collect();
            let mut charge = None;
            let mut status = None;
            
            for line in lines {
                if line.contains("EstimatedChargeRemaining") {
                    if let Some(value) = line.split(':').nth(1) {
                        charge = value.trim().parse::<u32>().ok();
                    }
                }
                if line.contains("BatteryStatus") {
                    if let Some(value) = line.split(':').nth(1) {
                        status = match value.trim() {
                            "1" => Some("Discharging"),
                            "2" => Some("Charging"),
                            "3" => Some("Fully Charged"),
                            _ => Some("Unknown"),
                        };
                    }
                }
            }
            
            match (charge, status) {
                (Some(c), Some(s)) => format!("{}% ({})", c, s),
                _ => "N/A (Desktop)".to_string(),
            }
        } else {
            "N/A (Desktop)".to_string()
        }
    }

    #[cfg(not(windows))]
    fn get_battery() -> String {
        if let Ok(output) = Command::new("cat")
            .args(["/sys/class/power_supply/BAT0/capacity"])
            .output()
        {
            let capacity_output = String::from_utf8_lossy(&output.stdout);
            let capacity = capacity_output.trim();
            if let Ok(output) = Command::new("cat")
                .args(["/sys/class/power_supply/BAT0/status"])
                .output()
            {
                let status_output = String::from_utf8_lossy(&output.stdout);
                let status = status_output.trim();
                return format!("{}% ({})", capacity, status);
            }
        }
        "N/A (Desktop)".to_string()
    }

    fn get_network_info() -> String {
        #[cfg(windows)]
        {
            if let Ok(output) = Command::new("powershell")
                .args(["-Command", "Get-NetAdapter | Where-Object {$_.Status -eq 'Up'} | Select-Object -First 1 Name, LinkSpeed"])
                .output()
            {
                let result = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = result.lines().collect();
                let mut name = None;
                let mut speed = None;
                
                for line in lines {
                    if line.contains("Name") && line.contains(":") {
                        name = line.split(':').nth(1).map(|s| s.trim());
                    }
                    if line.contains("LinkSpeed") && line.contains(":") {
                        speed = line.split(':').nth(1).map(|s| s.trim());
                    }
                }
                
                match (name, speed) {
                    (Some(n), Some(s)) => format!("{} ({})", n, s),
                    _ => "Unknown".to_string(),
                }
            } else {
                "Unknown".to_string()
            }
        }
        #[cfg(not(windows))]
        {
            // Try to get network interface info on Linux
            if let Ok(output) = Command::new("ip")
                .args(["route", "show", "default"])
                .output()
            {
                let result = String::from_utf8_lossy(&output.stdout);
                if let Some(line) = result.lines().next() {
                    if let Some(dev_pos) = line.find("dev ") {
                        let dev_part = &line[dev_pos + 4..];
                        if let Some(interface) = dev_part.split_whitespace().next() {
                            // Try to get speed info
                            if let Ok(speed_output) = Command::new("cat")
                                .arg(format!("/sys/class/net/{}/speed", interface))
                                .output()
                            {
                                let speed_raw = String::from_utf8_lossy(&speed_output.stdout);
                                let speed = speed_raw.trim();
                                if !speed.is_empty() && speed != "Unknown" {
                                    return format!("{} ({} Mbps)", interface, speed);
                                }
                            }
                            return interface.to_string();
                        }
                    }
                }
            }
            "Unknown".to_string()
        }
    }

    fn get_powershell_version() -> String {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "$PSVersionTable.PSVersion.ToString()"])
            .output()
        {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !version.is_empty() {
                return format!("PowerShell {}", version);
            }
        }
        "Unknown".to_string()
    }

    fn get_dotnet_version() -> String {
        if let Ok(output) = Command::new("dotnet")
            .args(["--version"])
            .output()
        {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !version.is_empty() {
                return format!(".NET {}", version);
            }
        }
        "Not installed".to_string()
    }

    #[cfg(windows)]
    fn get_desktop_environment() -> String {
        "DWM (Desktop Window Manager)".to_string()
    }

    #[cfg(not(windows))]
    fn get_desktop_environment() -> String {
        if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
            de
        } else if let Ok(de) = env::var("DESKTOP_SESSION") {
            de
        } else {
            "Unknown".to_string()
        }
    }

    #[cfg(windows)]
    fn get_system_language() -> String {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "Get-Culture | Select-Object -ExpandProperty DisplayName"])
            .output()
        {
            let lang = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !lang.is_empty() {
                return lang;
            }
        }
        "Unknown".to_string()
    }

    #[cfg(not(windows))]
    fn get_system_language() -> String {
        env::var("LANG").unwrap_or_else(|_| "Unknown".to_string())
    }

    fn get_active_processes(sys: &System) -> String {
        let process_count = sys.processes().len();
        format!("{} processes", process_count)
    }

    fn get_top_processes(sys: &System) -> String {
        let mut processes: Vec<_> = sys.processes().iter().collect();
        processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));
        
        let top_3: Vec<String> = processes
            .iter()
            .take(3)
            .map(|(_, process)| {
                format!("{} ({:.1}%)", process.name(), process.cpu_usage())
            })
            .collect();
        
        top_3.join(", ")
    }

    #[cfg(windows)]
    fn get_disk_speed() -> String {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", r"Get-Counter '\PhysicalDisk(_Total)\Disk Read Bytes/sec','\PhysicalDisk(_Total)\Disk Write Bytes/sec' -SampleInterval 1 -MaxSamples 1 | Select-Object -ExpandProperty CounterSamples | ForEach-Object { [math]::Round($_.CookedValue / 1MB, 2) }"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout);
            let speeds: Vec<&str> = result.lines().filter(|line| !line.trim().is_empty()).collect();
            if speeds.len() >= 2 {
                return format!("R: {} MB/s, W: {} MB/s", speeds[0].trim(), speeds[1].trim());
            }
        }
        "N/A".to_string()
    }

    #[cfg(not(windows))]
    fn get_disk_speed() -> String {
        "N/A".to_string()
    }

    #[cfg(windows)]
    fn get_network_usage() -> String {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", r"Get-Counter '\Network Interface(*)\Bytes Received/sec','\Network Interface(*)\Bytes Sent/sec' -SampleInterval 1 -MaxSamples 1 | Select-Object -ExpandProperty CounterSamples | Where-Object {$_.InstanceName -notlike '*Loopback*' -and $_.InstanceName -notlike '*isatap*'} | ForEach-Object { [math]::Round($_.CookedValue / 1KB, 2) } | Measure-Object -Sum | Select-Object -ExpandProperty Sum"])
            .output()
        {
            let usage_output = String::from_utf8_lossy(&output.stdout);
            let usage = usage_output.trim();
            if !usage.is_empty() {
                return format!("{} KB/s", usage);
            }
        }
        "N/A".to_string()
    }

    #[cfg(not(windows))]
    fn get_network_usage() -> String {
        // Try to get network usage from /proc/net/dev on Linux
        if let Ok(content) = std::fs::read_to_string("/proc/net/dev") {
            let mut total_rx = 0u64;
            let mut total_tx = 0u64;
            
            for line in content.lines().skip(2) { // Skip header lines
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    let interface = parts[0].trim_end_matches(':');
                    // Skip loopback and virtual interfaces
                    if !interface.starts_with("lo") && !interface.starts_with("docker") && !interface.starts_with("br-") {
                        if let (Ok(rx), Ok(tx)) = (parts[1].parse::<u64>(), parts[9].parse::<u64>()) {
                            total_rx += rx;
                            total_tx += tx;
                        }
                    }
                }
            }
            
            if total_rx > 0 || total_tx > 0 {
                return format!("RX: {:.2} MB, TX: {:.2} MB", total_rx as f64 / 1_048_576.0, total_tx as f64 / 1_048_576.0);
            }
        }
        "N/A".to_string()
    }

    pub fn format_info_sections(&self) -> Vec<String> {
        let mut sections = Vec::new();
        
        // OS Section
        sections.push(format!("{}", "OS".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2).bold()));
        sections.push(format!("{}", "─".repeat(50).truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2)));
        sections.push(format!("{} {}", "OS".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.os_name.white()));
        sections.push(format!("{} {}", "USER/HOST".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), format!("{}@{}", env::var("USERNAME").unwrap_or_else(|_| "user".to_string()), self.hostname).white()));
        sections.push(format!("{} {}", "KERNEL".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.kernel_version.white()));
        sections.push(format!("{} {}", "UPTIME".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.uptime.white()));
        sections.push(format!("{} {}", "SHELL".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.shell.white()));
        sections.push(format!("{} {}", "TERMINAL".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.terminal.white()));
        if !self.terminal_font.is_empty() && self.terminal_font != "Unknown" {
            sections.push(format!("{} {}", "FONT".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.terminal_font.white()));
        }
        if !self.windows_build.is_empty() && self.windows_build != "Unknown" && !self.windows_build.contains("N/A") {
            sections.push(format!("{} {}", "BUILD".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.windows_build.white()));
        }
        sections.push(format!("{} {}", "ARCH".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.architecture.white()));
        sections.push(format!("{} {}", "TIMEZONE".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.timezone.white()));
        sections.push("".to_string());
        
        // System Section
        sections.push(format!("{}", "SYSTEM".truecolor(CORAL.0, CORAL.1, CORAL.2).bold()));
        sections.push(format!("{}", "─".repeat(50).truecolor(CORAL.0, CORAL.1, CORAL.2)));
        sections.push(format!("{} {}", "PACKAGES".truecolor(CORAL.0, CORAL.1, CORAL.2), self.packages.white()));
        sections.push(format!("{} {}", "COLORS".truecolor(CORAL.0, CORAL.1, CORAL.2), self.color_scheme.white()));
        sections.push(format!("{} {}", "DE".truecolor(CORAL.0, CORAL.1, CORAL.2), self.desktop_environment.white()));
        sections.push(format!("{} {}", "LANGUAGE".truecolor(CORAL.0, CORAL.1, CORAL.2), self.system_language.white()));
        if !self.powershell_version.is_empty() && self.powershell_version != "Unknown" {
            sections.push(format!("{} {}", "POWERSHELL".truecolor(CORAL.0, CORAL.1, CORAL.2), self.powershell_version.white()));
        }
        if !self.dotnet_version.is_empty() && self.dotnet_version != "Not installed" {
            sections.push(format!("{} {}", ".NET".truecolor(CORAL.0, CORAL.1, CORAL.2), self.dotnet_version.white()));
        }
        if !self.manufacturer.is_empty() && self.manufacturer != "Unknown" {
            sections.push(format!("{} {}", "MANUFACTURER".truecolor(CORAL.0, CORAL.1, CORAL.2), self.manufacturer.white()));
        }
        if !self.model.is_empty() && self.model != "Unknown" {
            sections.push(format!("{} {}", "MODEL".truecolor(CORAL.0, CORAL.1, CORAL.2), self.model.white()));
        }
        sections.push("".to_string());
        
        // Hardware Section
        sections.push(format!("{}", "HARDWARE".truecolor(MINT.0, MINT.1, MINT.2).bold()));
        sections.push(format!("{}", "─".repeat(50).truecolor(MINT.0, MINT.1, MINT.2)));
        sections.push(format!("{} {}", "CPU".truecolor(MINT.0, MINT.1, MINT.2), self.cpu_info.white()));
        if !self.cpu_speed.is_empty() && self.cpu_speed != "Unknown" {
            sections.push(format!("{} {}", "CPU SPEED".truecolor(MINT.0, MINT.1, MINT.2), self.cpu_speed.white()));
        }
        if !self.cpu_temp.is_empty() && self.cpu_temp != "N/A" {
            sections.push(format!("{} {}", "CPU TEMP".truecolor(MINT.0, MINT.1, MINT.2), self.cpu_temp.white()));
        }
        sections.push(format!("{} {}", "CPU LOAD".truecolor(MINT.0, MINT.1, MINT.2), self.cpu_load.white()));
        sections.push(format!("{} {}", "GPU".truecolor(MINT.0, MINT.1, MINT.2), self.gpu_info.white()));
        if !self.gpu_temp.is_empty() && self.gpu_temp != "N/A" {
            sections.push(format!("{} {}", "GPU TEMP".truecolor(MINT.0, MINT.1, MINT.2), self.gpu_temp.white()));
        }
        if !self.gpu_vram.is_empty() && self.gpu_vram != "Unknown" {
            sections.push(format!("{} {}", "GPU VRAM".truecolor(MINT.0, MINT.1, MINT.2), self.gpu_vram.white()));
        }
        sections.push(format!("{} {}", "MEMORY".truecolor(MINT.0, MINT.1, MINT.2), self.memory_info.white()));
        sections.push(format!("{} {}", "DISK".truecolor(MINT.0, MINT.1, MINT.2), self.disk_info.white()));
        if !self.monitors.is_empty() && self.monitors != "Unknown" {
            sections.push(format!("{} {}", "MONITORS".truecolor(MINT.0, MINT.1, MINT.2), self.monitors.white()));
        }
        if !self.battery.contains("N/A") {
            sections.push(format!("{} {}", "BATTERY".truecolor(MINT.0, MINT.1, MINT.2), self.battery.white()));
        }
        sections.push("".to_string());
        
        // Network Section
        sections.push(format!("{}", "NETWORK".truecolor(SKY_BLUE.0, SKY_BLUE.1, SKY_BLUE.2).bold()));
        sections.push(format!("{}", "─".repeat(50).truecolor(SKY_BLUE.0, SKY_BLUE.1, SKY_BLUE.2)));
        if !self.network_info.is_empty() && self.network_info != "Unknown" {
            sections.push(format!("{} {}", "ADAPTER".truecolor(SKY_BLUE.0, SKY_BLUE.1, SKY_BLUE.2), self.network_info.white()));
        }
        if !self.network_usage.is_empty() && self.network_usage != "N/A" {
            sections.push(format!("{} {}", "USAGE".truecolor(SKY_BLUE.0, SKY_BLUE.1, SKY_BLUE.2), self.network_usage.white()));
        }
        sections.push("".to_string());
        
        // Statistics Section
        sections.push(format!("{}", "STATISTICS".truecolor(PURPLE.0, PURPLE.1, PURPLE.2).bold()));
        sections.push(format!("{}", "─".repeat(50).truecolor(PURPLE.0, PURPLE.1, PURPLE.2)));
        sections.push(format!("{} {}", "PROCESSES".truecolor(PURPLE.0, PURPLE.1, PURPLE.2), self.active_processes.white()));
        sections.push(format!("{} {}", "TOP CPU".truecolor(PURPLE.0, PURPLE.1, PURPLE.2), self.top_processes.white()));
        if !self.disk_speed.is_empty() && self.disk_speed != "N/A" {
            sections.push(format!("{} {}", "DISK I/O".truecolor(PURPLE.0, PURPLE.1, PURPLE.2), self.disk_speed.white()));
        }
        // Additional stats content
        sections.push(format!("{} {}", "THREADS".truecolor(PURPLE.0, PURPLE.1, PURPLE.2), format!("{}", std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)).white()));
        sections.push(format!("{} {}", "BOOT TIME".truecolor(PURPLE.0, PURPLE.1, PURPLE.2), format!("{:.2}s", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as f64 - self.uptime.split_whitespace().next().unwrap_or("0").parse::<f64>().unwrap_or(0.0) * 86400.0).white()));
        sections.push("".to_string());
        
        sections
    }

    pub fn format_info_sections_filtered(&self, filter: &SectionFilter) -> Vec<String> {
        let mut sections = Vec::new();
        
        // OS Section
        if filter.show_os {
            sections.push(format!("{}", "OS".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2).bold()));
            sections.push(format!("{}", "─".repeat(50).truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2)));
            sections.push(format!("{} {}", "OS".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.os_name.white()));
            sections.push(format!("{} {}", "USER/HOST".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), format!("{}@{}", env::var("USERNAME").unwrap_or_else(|_| "user".to_string()), self.hostname).white()));
            sections.push(format!("{} {}", "KERNEL".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.kernel_version.white()));
            sections.push(format!("{} {}", "UPTIME".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.uptime.white()));
            sections.push(format!("{} {}", "SHELL".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.shell.white()));
            sections.push(format!("{} {}", "TERMINAL".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.terminal.white()));
            if !self.terminal_font.is_empty() && self.terminal_font != "Unknown" {
                sections.push(format!("{} {}", "FONT".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.terminal_font.white()));
            }
            if !self.windows_build.is_empty() && self.windows_build != "Unknown" && !self.windows_build.contains("N/A") {
                sections.push(format!("{} {}", "BUILD".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.windows_build.white()));
            }
            sections.push(format!("{} {}", "ARCH".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.architecture.white()));
            sections.push(format!("{} {}", "TIMEZONE".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.timezone.white()));
            sections.push("".to_string());
        }
        
        // System Section
        if filter.show_system {
            sections.push(format!("{}", "SYSTEM".truecolor(CORAL.0, CORAL.1, CORAL.2).bold()));
            sections.push(format!("{}", "─".repeat(50).truecolor(CORAL.0, CORAL.1, CORAL.2)));
            sections.push(format!("{} {}", "PACKAGES".truecolor(CORAL.0, CORAL.1, CORAL.2), self.packages.white()));
            sections.push(format!("{} {}", "COLORS".truecolor(CORAL.0, CORAL.1, CORAL.2), self.color_scheme.white()));
            sections.push(format!("{} {}", "DE".truecolor(CORAL.0, CORAL.1, CORAL.2), self.desktop_environment.white()));
            sections.push(format!("{} {}", "LANGUAGE".truecolor(CORAL.0, CORAL.1, CORAL.2), self.system_language.white()));
            if !self.powershell_version.is_empty() && self.powershell_version != "Unknown" {
                sections.push(format!("{} {}", "POWERSHELL".truecolor(CORAL.0, CORAL.1, CORAL.2), self.powershell_version.white()));
            }
            if !self.dotnet_version.is_empty() && self.dotnet_version != "Not installed" {
                sections.push(format!("{} {}", ".NET".truecolor(CORAL.0, CORAL.1, CORAL.2), self.dotnet_version.white()));
            }
            if !self.manufacturer.is_empty() && self.manufacturer != "Unknown" {
                sections.push(format!("{} {}", "MANUFACTURER".truecolor(CORAL.0, CORAL.1, CORAL.2), self.manufacturer.white()));
            }
            if !self.model.is_empty() && self.model != "Unknown" {
                sections.push(format!("{} {}", "MODEL".truecolor(CORAL.0, CORAL.1, CORAL.2), self.model.white()));
            }
            sections.push("".to_string());
        }
        
        // Hardware Section
        if filter.show_hardware {
            sections.push(format!("{}", "HARDWARE".truecolor(MINT.0, MINT.1, MINT.2).bold()));
            sections.push(format!("{}", "─".repeat(50).truecolor(MINT.0, MINT.1, MINT.2)));
            sections.push(format!("{} {}", "CPU".truecolor(MINT.0, MINT.1, MINT.2), self.cpu_info.white()));
            if !self.cpu_speed.is_empty() && self.cpu_speed != "Unknown" {
                sections.push(format!("{} {}", "CPU SPEED".truecolor(MINT.0, MINT.1, MINT.2), self.cpu_speed.white()));
            }
            if !self.cpu_temp.is_empty() && self.cpu_temp != "N/A" {
                sections.push(format!("{} {}", "CPU TEMP".truecolor(MINT.0, MINT.1, MINT.2), self.cpu_temp.white()));
            }
            sections.push(format!("{} {}", "CPU LOAD".truecolor(MINT.0, MINT.1, MINT.2), self.cpu_load.white()));
            sections.push(format!("{} {}", "GPU".truecolor(MINT.0, MINT.1, MINT.2), self.gpu_info.white()));
            if !self.gpu_temp.is_empty() && self.gpu_temp != "N/A" {
                sections.push(format!("{} {}", "GPU TEMP".truecolor(MINT.0, MINT.1, MINT.2), self.gpu_temp.white()));
            }
            if !self.gpu_vram.is_empty() && self.gpu_vram != "Unknown" {
                sections.push(format!("{} {}", "GPU VRAM".truecolor(MINT.0, MINT.1, MINT.2), self.gpu_vram.white()));
            }
            sections.push(format!("{} {}", "MEMORY".truecolor(MINT.0, MINT.1, MINT.2), self.memory_info.white()));
            sections.push(format!("{} {}", "DISK".truecolor(MINT.0, MINT.1, MINT.2), self.disk_info.white()));
            if !self.monitors.is_empty() && self.monitors != "Unknown" {
                sections.push(format!("{} {}", "MONITORS".truecolor(MINT.0, MINT.1, MINT.2), self.monitors.white()));
            }
            if !self.battery.contains("N/A") {
                sections.push(format!("{} {}", "BATTERY".truecolor(MINT.0, MINT.1, MINT.2), self.battery.white()));
            }
            sections.push("".to_string());
        }
        
        // Network Section
        if filter.show_network {
            sections.push(format!("{}", "NETWORK".truecolor(SKY_BLUE.0, SKY_BLUE.1, SKY_BLUE.2).bold()));
            sections.push(format!("{}", "─".repeat(50).truecolor(SKY_BLUE.0, SKY_BLUE.1, SKY_BLUE.2)));
            if !self.network_info.is_empty() && self.network_info != "Unknown" {
                sections.push(format!("{} {}", "ADAPTER".truecolor(SKY_BLUE.0, SKY_BLUE.1, SKY_BLUE.2), self.network_info.white()));
            }
            if !self.network_usage.is_empty() && self.network_usage != "N/A" {
                sections.push(format!("{} {}", "USAGE".truecolor(SKY_BLUE.0, SKY_BLUE.1, SKY_BLUE.2), self.network_usage.white()));
            }
            sections.push("".to_string());
        }
        
        // Statistics Section
        if filter.show_stats {
            sections.push(format!("{}", "STATISTICS".truecolor(PURPLE.0, PURPLE.1, PURPLE.2).bold()));
            sections.push(format!("{}", "─".repeat(50).truecolor(PURPLE.0, PURPLE.1, PURPLE.2)));
            sections.push(format!("{} {}", "PROCESSES".truecolor(PURPLE.0, PURPLE.1, PURPLE.2), self.active_processes.white()));
            sections.push(format!("{} {}", "TOP CPU".truecolor(PURPLE.0, PURPLE.1, PURPLE.2), self.top_processes.white()));
            if !self.disk_speed.is_empty() && self.disk_speed != "N/A" {
                sections.push(format!("{} {}", "DISK I/O".truecolor(PURPLE.0, PURPLE.1, PURPLE.2), self.disk_speed.white()));
            }
            // Additional stats content
            sections.push(format!("{} {}", "THREADS".truecolor(PURPLE.0, PURPLE.1, PURPLE.2), format!("{}", std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)).white()));
            sections.push(format!("{} {}", "BOOT TIME".truecolor(PURPLE.0, PURPLE.1, PURPLE.2), format!("{:.2}s", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as f64 - self.uptime.split_whitespace().next().unwrap_or("0").parse::<f64>().unwrap_or(0.0) * 86400.0).white()));
            sections.push("".to_string());
        }
        
        sections
    }

    pub fn format_info_sections_summary(&self) -> Vec<String> {
        let mut sections = Vec::new();
        
        // OS Section - Essential info only
        sections.push(format!("{}", "OS".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2).bold()));
        sections.push(format!("{}", "─".repeat(50).truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2)));
        sections.push(format!("{} {}", "OS".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.os_name.white()));
        sections.push(format!("{} {}", "USER/HOST".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), format!("{}@{}", env::var("USERNAME").unwrap_or_else(|_| "user".to_string()), self.hostname).white()));
        sections.push(format!("{} {}", "SHELL".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.shell.white()));
        sections.push(format!("{} {}", "TERMINAL".truecolor(WARM_YELLOW.0, WARM_YELLOW.1, WARM_YELLOW.2), self.terminal.white()));
        sections.push("".to_string());
        
        // System Section - Essential info only
        sections.push(format!("{}", "SYSTEM".truecolor(CORAL.0, CORAL.1, CORAL.2).bold()));
        sections.push(format!("{}", "─".repeat(50).truecolor(CORAL.0, CORAL.1, CORAL.2)));
        sections.push(format!("{} {}", "PACKAGES".truecolor(CORAL.0, CORAL.1, CORAL.2), self.packages.white()));
        sections.push(format!("{} {}", "COLORS".truecolor(CORAL.0, CORAL.1, CORAL.2), self.color_scheme.white()));
        sections.push("".to_string());
        
        // Hardware Section - Essential info only
        sections.push(format!("{}", "HARDWARE".truecolor(MINT.0, MINT.1, MINT.2).bold()));
        sections.push(format!("{}", "─".repeat(50).truecolor(MINT.0, MINT.1, MINT.2)));
        sections.push(format!("{} {}", "CPU".truecolor(MINT.0, MINT.1, MINT.2), self.cpu_info.white()));
        sections.push(format!("{} {}", "GPU".truecolor(MINT.0, MINT.1, MINT.2), self.gpu_info.white()));
        sections.push(format!("{} {}", "MEMORY".truecolor(MINT.0, MINT.1, MINT.2), self.memory_info.white()));
        sections.push(format!("{} {}", "DISK".truecolor(MINT.0, MINT.1, MINT.2), self.disk_info.white()));
        sections.push("".to_string());
        
        sections
    }

    pub fn display_color_palette() -> Vec<String> {
        let mut palette = Vec::new();
        
        // Palette section header
        palette.push(format!("{}", "Palette".truecolor(SKY_BLUE.0, SKY_BLUE.1, SKY_BLUE.2).bold()));
        palette.push(format!("{}", "─".repeat(50).truecolor(SKY_BLUE.0, SKY_BLUE.1, SKY_BLUE.2)));
        
        // Terminal colors as circles
        let colors = [
            (0, 0, 0),       // black
            (128, 0, 0),     // red
            (0, 128, 0),     // green
            (128, 128, 0),   // yellow
            (0, 0, 128),     // blue
            (128, 0, 128),   // magenta
            (0, 128, 128),   // cyan
            (192, 192, 192), // white
        ];
        
        let bright_colors = [
            (128, 128, 128), // bright black
            (255, 0, 0),     // bright red
            (0, 255, 0),     // bright green
            (255, 255, 0),   // bright yellow
            (0, 0, 255),     // bright blue
            (255, 0, 255),   // bright magenta
            (0, 255, 255),   // bright cyan
            (255, 255, 255), // bright white
        ];
        
        // Display normal colors as circles
        let mut normal_line = String::new();
        for (r, g, b) in &colors {
            normal_line.push_str(&format!("{} ", "●".truecolor(*r, *g, *b)));
        }
        palette.push(normal_line);
        
        // Display bright colors as circles
        let mut bright_line = String::new();
        for (r, g, b) in &bright_colors {
            bright_line.push_str(&format!("{} ", "●".truecolor(*r, *g, *b)));
        }
        palette.push(bright_line);
        
        palette
    }
}