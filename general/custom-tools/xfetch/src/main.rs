use std::time::Instant;
use colored::*;
use sysinfo::System;
use std::env;

mod logos;
mod system_info;

use logos::get_logo;
use system_info::SystemInfo;

fn main() {
    let start_time = Instant::now();
    
    // Initialize system information
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let system_info = SystemInfo::new(&sys);
    
    // Display the system information with new layout
    display_system_info(&system_info, start_time);
}

fn display_system_info(info: &SystemInfo, start_time: Instant) {
    let logo = get_logo(&info.os_name);
    let info_lines = info.format_info();
    let palette_lines = SystemInfo::display_color_palette();
    
    let logo_lines: Vec<&str> = logo.lines().collect();
    
    // Calculate the maximum width needed for logo
    let logo_width = logo_lines.iter().map(|line| line.len()).max().unwrap_or(0) + 5;
    
    // Combine all right-side content: system info + palette + fetch time
    let mut right_column = Vec::new();
    
    // Add system info lines
    for info_line in info_lines.iter() {
        right_column.push(info_line.clone());
    }
    
    // Add empty line separator
    right_column.push("".to_string());
    
    // Add palette lines
    for line in palette_lines {
        if !line.is_empty() {
            right_column.push(line);
        }
    }
    
    // Add empty line separator
    right_column.push("".to_string());
    
    // Add fetch time
    let elapsed = start_time.elapsed();
    right_column.push(format!("{} {:.2}ms", "Fetch time:".bright_black(), elapsed.as_millis()));
    
    let max_lines = std::cmp::max(logo_lines.len(), right_column.len());
    
    for i in 0..max_lines {
        let logo_line = logo_lines.get(i).unwrap_or(&"");
        let empty_string = String::new();
        let right_line = right_column.get(i).unwrap_or(&empty_string);
        
        println!("{:<width$} {}", logo_line, right_line, width = logo_width);
    }
}