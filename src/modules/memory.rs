use crate::utils::format;
use sysinfo::{System, SystemExt};

pub fn get_total_memory() -> String {
    let mut system = System::new_all();
    system.refresh_memory();
    format::bytes(system.total_memory())
}

pub fn get_used_memory() -> String {
    let mut system = System::new_all();
    system.refresh_memory();
    format::bytes(system.used_memory())
}

pub fn get_memory_percentage() -> String {
    let mut system = System::new_all();
    system.refresh_memory();
    let total = system.total_memory();
    let used = system.used_memory();
    
    if total == 0 {
        return "0%".to_string();
    }
    
    format::percentage(used, total)
}

pub fn get_swap_total() -> String {
    let mut system = System::new_all();
    system.refresh_memory();
    format::bytes(system.total_swap())
}

pub fn get_swap_used() -> String {
    let mut system = System::new_all();
    system.refresh_memory();
    format::bytes(system.used_swap())
}

pub fn get_swap_percentage() -> String {
    let mut system = System::new_all();
    system.refresh_memory();
    let total = system.total_swap();
    let used = system.used_swap();
    
    if total == 0 {
        return "0%".to_string();
    }
    
    format::percentage(used, total)
}