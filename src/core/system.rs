use serde::{Deserialize, Serialize};
use crate::modules::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub cpu: String,
    pub memory: MemoryInfo,
    pub disk: DiskInfo,
    pub gpu: String,
    pub os: OSInfo,
    pub kernel: String,
    pub package_manager_version: String,
    pub uptime: String,
    pub datetime: String,
    pub shell: String,
    pub terminal: String,
    pub network: NetworkInfo,
    pub packages: PackageInfo,
    pub compositor: String,
    pub drivers: String,
    pub media: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: String,
    pub used: String,
    pub percentage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub total: String,
    pub used: String,
    pub percentage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub local_ip: String,
    pub interface: String,
    pub extra_info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub nix_system: String,
    pub nix_user: String,
    pub total: String,
    pub breakdown: String,
}

pub struct SystemCollector;

impl SystemCollector {
    pub fn collect_all() -> SystemInfo {
        SystemInfo {
            cpu: cpu::get_cpu_info(),
            memory: MemoryInfo {
                total: memory::get_total_memory(),
                used: memory::get_used_memory(),
                percentage: memory::get_memory_percentage(),
            },
            disk: DiskInfo {
                total: disk::get_total_disk(),
                used: disk::get_used_disk(),
                percentage: disk::get_disk_percentage(),
            },
            gpu: gpu::get_gpu_info(),
            os: OSInfo {
                name: os::get_os_name(),
                version: os::get_os_version(),
                arch: os::get_os_arch(),
            },
            kernel: kernel::get_kernel_version(),
            package_manager_version: packages::get_package_manager_version(),
            uptime: uptime::get_uptime(),
            datetime: datetime::get_datetime(),
            shell: shell::get_shell_info(),
            terminal: terminal::get_terminal_info(),
            network: NetworkInfo {
                local_ip: network::get_local_ip(),
                interface: network::get_interface(),
                extra_info: network::get_extra_info(),
            },
            packages: PackageInfo {
                nix_system: packages::get_nix_system_packages(),
                nix_user: packages::get_nix_user_packages(),
                total: packages::get_total_packages(),
                breakdown: packages::get_package_breakdown(),
            },
            compositor: compositor::get_compositor_info(),
            drivers: drivers::get_driver_info(),
            media: media::get_media_info(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSInfo {
    pub name: String,
    pub version: String,
    pub arch: String,
}