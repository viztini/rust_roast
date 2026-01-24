use sysinfo::{CpuExt, System, SystemExt};
use colored::*;
use rand::seq::SliceRandom;
use std::process::Command;

fn get_gpu_name() -> String {
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("lspci -v | grep -i vga | grep -i 'controller\\|display'")
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next() {
            if let Some(start) = line.find(": ") {
                let name_part = &line[start + 2..];
                if let Some(end) = name_part.find(" (") {
                    return name_part[..end].trim().to_string();
                }
                return name_part.trim().to_string();
            }
            return line.trim().to_string();
        }
    }
    "Unknown GPU".to_string()
}

fn is_laptop() -> bool {
    std::path::Path::new("/sys/class/power_supply/BAT0").exists() ||
    std::path::Path::new("/sys/class/power_supply/BAT1").exists()
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", "\n--- System Specs ---".cyan().bold());

    let cpu_count = sys.cpus().len();
    let (cpu_brand, cpu_frequency) = if cpu_count > 0 {
        (sys.cpus()[0].brand(), sys.cpus()[0].frequency())
    } else {
        ("Unknown CPU", 0)
    };
    println!(
        "  {}: {} ({} cores @ {} MHz)",
        "CPU".green().bold(),
        cpu_brand.white(),
        cpu_count.to_string().white(),
        cpu_frequency.to_string().white()
    );

    let total_ram_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_ram_gb = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    println!(
        "  {}: {:.2} GB total, {:.2} GB used",
        "RAM".green().bold(),
        total_ram_gb,
        used_ram_gb
    );

    let gpu_name = get_gpu_name();
    println!("  {}: {}", "GPU".green().bold(), gpu_name.white());

    let os_name = sys.name().unwrap_or_else(|| "Unknown OS".to_string());
    let os_version = sys.os_version().unwrap_or_else(|| "Unknown Version".to_string());
    println!("  {}: {} {}", "OS".green().bold(), os_name.white(), os_version.white());

    let form_factor = if is_laptop() { "Laptop" } else { "Desktop" };
    println!("  {}: {}", "Form Factor".green().bold(), form_factor.white());

    println!("{}", "\n--- The Roast ---".red().bold());

    let mut roasts = Vec::new();
    let mut rng = rand::thread_rng();

    let cpu_roasts = vec!["Your CPU is slow...", "Your CPU is decent..."];
    let ram_roasts = vec!["Your RAM is tiny...", "Your RAM is fine..."];
    let gpu_roasts = vec!["Your GPU struggles...", "Your GPU is fine..."];
    let laptop_roasts = vec!["Ah, a laptop user.", "Portable problems."];
    let desktop_roasts = vec!["Desktop user detected.", "Immovable problems."];

    if cpu_count < 4 || cpu_frequency < 2000 {
        roasts.push(cpu_roasts.choose(&mut rng).unwrap().to_string());
    }

    if total_ram_gb < 8.0 {
        roasts.push(ram_roasts.choose(&mut rng).unwrap().to_string());
    }

    if (gpu_name.contains("Integrated") || gpu_name.contains("Intel") || gpu_name.contains("AMD Radeon Graphics"))
        && !gpu_name.contains("RX")
    {
        roasts.push(gpu_roasts.choose(&mut rng).unwrap().to_string());
    }

    if form_factor == "Laptop" {
        roasts.push(laptop_roasts.choose(&mut rng).unwrap().to_string());
    } else {
        roasts.push(desktop_roasts.choose(&mut rng).unwrap().to_string());
    }

    for roast in roasts {
        println!("  {}", roast.yellow());
    }
    println!();
}
