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
    invert: i32,
    threshold: i32,
    resource: &mut resource::Resource,
) -> bool {
    match res_option {
        resource::ResourceOptions::ProcMem => invert * ((*resource).proc_memory() - threshold) > 0,
        resource::ResourceOptions::SysMem  => invert * ((*resource).sys_mem_percentage() - threshold) > 0,
        resource::ResourceOptions::CpuUtil => invert * ((*resource).cpu_util() - threshold) > 0,
        resource::ResourceOptions::CpuTemp => invert * ((*resource).cpu_temp() - threshold) > 0,
        resource::ResourceOptions::RunTime => invert * ((*resource).run_time() - threshold) > 0,
    }
}

pub fn monitor(resource: &mut resource::Resource, level: i32, res_option: resource::ResourceOptions, invert: i32, threshold: i32) {
    info!("Started monitoring PID {} for {}", resource.pid(), res_option);
    let mut tries = 0;
    let mut current_level = level;
    let mut timeout: u64 = 1;
    // send sigkill only once
    while resource.pid_exists() && current_level <= 2 {
 
        if should_kill(&res_option, invert, threshold, resource) {
            resource.maybe_kill(current_level);
            // don't sleep if process died or sigkill is sent
            if resource.pid_exists() && current_level < 2 {
                std::thread::sleep(std::time::Duration::from_secs(timeout));
            }
            escalate_level(&mut current_level, &mut tries, &mut timeout);
        }
    }
    info!("Ending monitoring PID {}", resource.pid());
    info!("PID {} not alive", resource.pid());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escalation() {
        let (mut level, mut tries, mut timeout) = (0, 0, 1);

        // increase tries, but no change to level and timeout just yet
        escalate_level(&mut level, &mut tries, &mut timeout);
        assert_eq!((level, tries, timeout), (0, 1, 1));

        // increase all
        escalate_level(&mut level, &mut tries, &mut timeout);
        assert_eq!((level, tries, timeout), (1, 2, 2));
    }

    #[test]
    fn test_should_kill() {
        // create dependency
        let mut invalid_resource = resource::Resource::new(-1);

        assert!(should_kill(&resource::ResourceOptions::SysMem, 1, 0, &mut invalid_resource));
        assert!(!should_kill(&resource::ResourceOptions::SysMem, 1, 1000, &mut invalid_resource));
    }
}
