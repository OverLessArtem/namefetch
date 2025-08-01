use crate::config::{BLACK, LIGHT_GRAY, WHITE, LIGHT_BLUE, NC, GAP};
use namefetch::{get_cpu_model, get_gpu_model, get_disk_info, get_shell, get_package_count, get_memory_info, Uptime};
use std::env;

mod config;

fn get_string_length(s: &str) -> usize {
    let mut result = 0;
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(c) = chars.next() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result += 1;
        }
    }
    result
}

fn get_info() -> Vec<String> {
    let mut info_lines = Vec::new();

    let hostname = sys_info::hostname().unwrap_or("?".to_string());
    let username = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let distro = sys_info::linux_os_release().unwrap().pretty_name.unwrap_or("?".to_string());
    let kernel_version = sys_info::os_release().unwrap_or("?".to_string());
    let uptime = Uptime::new().unwrap();
    let package_count = get_package_count()
        .map(|n| format!("{} (терабайт порно)", n))
        .unwrap_or_else(|| "? (терабайт порно)".to_string());
    let cpu = get_cpu_model().map(|s| format!("{} (ГОВНО)", s)).unwrap_or("?".to_string());
    let gpu = get_gpu_model().map(|s| format!("{} (ГОВНО)", s)).unwrap_or("?".to_string());
    let disk = get_disk_info().unwrap_or("?".to_string());
    let shell = get_shell().unwrap_or("?".to_string());
    let memory = get_memory_info().unwrap_or("?".to_string());

    info_lines.push(format!("{}{}{}@{}{}{}", 
        WHITE, username, LIGHT_GRAY, LIGHT_BLUE, hostname, NC));
    
    info_lines.push(format!("{}{}{}",
        LIGHT_GRAY, "─".repeat(45), NC));
    
    info_lines.push(format!("{} {}ТВОЕ ИМЯ НА ЯЗЫКЕ ЛИНУКС{}", LIGHT_BLUE, WHITE, NC));
    info_lines.push(format!("{}├─ {}Налогоплательщик{} {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, username, NC));
    info_lines.push(format!("{}├─ {}Номер карты{}     {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, distro, NC));
    info_lines.push(format!("{}├─ {}Версия Линукса{}  {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, kernel_version, NC));
    info_lines.push(format!("{}├─ {}Играл{}         {}{}назад{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, uptime.get(), NC));
    info_lines.push(format!("{}├─ {}Биткоины{}      {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, package_count, NC));
    info_lines.push(format!("{}└─ {}Вещество{}      {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, shell, NC));
    
    info_lines.push("".to_string());
    
    info_lines.push(format!("{} {}Железо{}", LIGHT_BLUE, WHITE, NC));
    info_lines.push(format!("{}├─ {}ЦП{}           {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, cpu, NC));
    info_lines.push(format!("{}├─ {}ГПУ{}          {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, gpu, NC));
    info_lines.push(format!("{}├─ {}Оперативка{}   {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, memory, NC));
    info_lines.push(format!("{}└─ {}Флэш-память{}   {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, disk, NC));
    
    info_lines.push("".to_string());
    
    let mut palette = format!("{}Цвета: ", LIGHT_GRAY);
    let colors = [BLACK, LIGHT_GRAY, WHITE, LIGHT_BLUE];
    for &color in &colors {
        palette.push_str(&format!("{}███{}", color, NC));
    }
    info_lines.push(palette);

    info_lines
}

fn main() {
    let logo = config::get_logo();
    let info_lines = get_info();

    let mut max_logo_width = 0;
    for line in &logo {
        let current_width = get_string_length(line);
        if current_width > max_logo_width {
            max_logo_width = current_width;
        }
    }

    println!();

    let max_lines = logo.len().max(info_lines.len());

    for i in 0..max_lines {
        if i < logo.len() {
            let logo_line = &logo[i];
            print!("{}", logo_line);
            let logo_line_width = get_string_length(logo_line);
            let padding = max_logo_width - logo_line_width + GAP;
            print!("{}", " ".repeat(padding));
        } else {
            print!("{}", " ".repeat(max_logo_width + GAP));
        }
        
        if i < info_lines.len() {
            println!("{}", info_lines[i]);
        } else {
            println!();
        }
    }

    println!();
}
