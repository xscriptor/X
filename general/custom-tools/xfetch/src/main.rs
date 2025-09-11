use std::time::Instant;
use colored::*;
use sysinfo::System;
use clap::Parser;

mod system_info;

use system_info::SystemInfo;

#[derive(Parser)]
#[command(name = "xfetch")]
#[command(about = "A system information tool written in Rust")]
struct Args {
    /// Show OS information
    #[arg(short = 'O', long = "os")]
    os: bool,
    
    /// Show System information
    #[arg(short = 'S', long = "system")]
    system: bool,
    
    /// Show Hardware information
    #[arg(short = 'H', long = "hardware")]
    hardware: bool,
    
    /// Show Network information
    #[arg(short = 'N', long = "network")]
    network: bool,
    
    /// Show Statistics information
    #[arg(short = 'P', long = "stats")]
    stats: bool,
    
    /// Show color palette
    #[arg(short = 'C', long = "colors")]
    colors: bool,
    
    /// Show summary with essential information only
    #[arg(short = 'X', long = "summary")]
    summary: bool,
}

#[derive(Debug, Clone)]
pub struct SectionFilter {
    pub show_os: bool,
    pub show_system: bool,
    pub show_hardware: bool,
    pub show_network: bool,
    pub show_stats: bool,
    pub show_palette: bool,
    pub summary_mode: bool,
}

fn main() {
    let start_time = Instant::now();
    let args = Args::parse();
    
    // Create section filter based on arguments
    let filter = create_section_filter(&args);
    
    // Initialize system information
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let system_info = SystemInfo::new_filtered(&sys, &filter);
    
    // Display the system information with filtering
    display_system_info(&system_info, start_time, &filter);
}

fn create_section_filter(args: &Args) -> SectionFilter {
    // If summary mode is enabled, only show essential information
    if args.summary {
        return SectionFilter {
            show_os: true,
            show_system: true,
            show_hardware: false,
            show_network: false,
            show_stats: false,
            show_palette: false,
            summary_mode: true,
        };
    }
    
    // If no specific flags are provided, show all sections
    let show_all = !args.os && !args.system && !args.hardware && !args.network && !args.stats && !args.colors;
    
    SectionFilter {
        show_os: show_all || args.os,
        show_system: show_all || args.system,
        show_hardware: show_all || args.hardware,
        show_network: show_all || args.network,
        show_stats: show_all || args.stats,
        show_palette: show_all || args.colors,
        summary_mode: false,
    }
}

fn display_system_info(info: &SystemInfo, start_time: Instant, filter: &SectionFilter) {
    let info_sections = if filter.summary_mode {
        info.format_info_sections_summary()
    } else {
        info.format_info_sections_filtered(filter)
    };
    
    // Display system info sections
    for section_line in info_sections.iter() {
        println!("{}", section_line);
    }
    
    // Display palette section if requested
    if filter.show_palette {
        let palette_lines = SystemInfo::display_color_palette();
        for line in palette_lines {
            println!("{}", line);
        }
    }
    
    // Always show palette in summary mode
    if filter.summary_mode {
        let palette_lines = SystemInfo::display_color_palette();
        for line in palette_lines {
            println!("{}", line);
        }
    }
    
    // Add empty line separator
    println!();
    
    // Display fetch time with purple color
    let elapsed = start_time.elapsed();
    println!("{} {:.2}ms", "Fetch time:".truecolor(148, 138, 227), elapsed.as_millis());
}