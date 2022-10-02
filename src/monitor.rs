use std::{thread, time::Duration, path::Path, process::Command};
use tracing::{info, warn};

use crate::mem;

fn maybe_stop(pid: i32, level: &mut i32) {
    let signal = if *level == 0 { "INT" } else if *level == 1 { "TERM" } else { "KILL" };
    info!("Attempting signal {} on pid {}", signal, pid);
    Command::new("sh")
             .arg("-c")
             .arg(format!("kill -{signal} {pid}"))
             .output().expect("Failed to execute kill");
}

fn escalate_level(level: &mut i32, tries: &mut i32, timeout: &mut u64) {
    if *tries % 2== 0 {
        *level += 1;
        *timeout *= 2;
        warn!("Escalating signal level, increasing timeout to {}", timeout);
    }
    *tries += 1;
}

pub fn monitor(pid: i32, level: i32, sys_threshold: i32, process_threshold: i32) {
    info!("Started monitoring memory usage of pid {}", pid);
    let mut tries = 0;
    let mut current_level = level;
    let mut timeout: u64 = 1;
    while Path::new(&format!("/proc/{pid}")).exists() {
        
        if mem::read_private_memory(pid) > process_threshold 
            || mem::read_system_memory_percentage() > sys_threshold {
            maybe_stop(pid, &mut current_level);
            escalate_level(&mut current_level, &mut tries, &mut timeout);
        }
        thread::sleep(Duration::from_secs(timeout))
    }
    info!("Ending monitoring memory usage of pid {}", pid);
    info!("PID {} not alive / killed", pid);
}
