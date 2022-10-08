use tracing::{info, warn};
use crate::resource;

fn escalate_level(level: &mut i32, tries: &mut i32, timeout: &mut u64) {
    *tries += 1;
    if *tries % 2 == 0 {
        *level += 1;
        *timeout *= 2;
        warn!("Escalating signal level, increasing timeout to {}", timeout);
    }
}

fn should_kill(
    res_option: &resource::ResourceOptions,
    more: i32,
    threshold: i32,
    resource: &mut resource::Resource,
) -> bool {
    match res_option {
        resource::ResourceOptions::ProcMem => more * ((*resource).proc_memory() - threshold) > 0,
        resource::ResourceOptions::SysMem  => more * ((*resource).sys_mem_percentage() - threshold) > 0,
        resource::ResourceOptions::CpuUtil => more * ((*resource).cpu_util() - threshold) > 0,
        resource::ResourceOptions::CpuTemp => more * ((*resource).cpu_temp() - threshold) > 0,
    }
}

pub fn monitor(pid: i32, level: i32, res_option: resource::ResourceOptions, more: i32, threshold: i32) {
    info!("Started monitoring PID {} for {}", pid, res_option);
    let mut tries = 0;
    let mut current_level = level;
    let mut timeout: u64 = 1;
    let mut resource = resource::Resource::new(pid);
    while resource.pid_exists() {
        
        if should_kill(&res_option, more, threshold, &mut resource) {
            resource.maybe_kill(current_level);
            escalate_level(&mut current_level, &mut tries, &mut timeout);
        }

        std::thread::sleep(std::time::Duration::from_secs(timeout))
    }
    info!("Ending monitoring PID {}", pid);
    info!("PID {} not alive", pid);
}
