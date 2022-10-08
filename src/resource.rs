use std::{fs, fmt};
use tracing::{info, warn};
use sysinfo::{SystemExt, System, CpuExt, ComponentExt, Pid, ProcessExt};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ResourceOptions {
    ProcMem,
    SysMem,
    CpuUtil,
    CpuTemp,
}

impl fmt::Display for ResourceOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct Resource {
    pid: Pid,
    system: System,
}

impl Resource {
    pub fn new(pid: i32) -> Resource {
        Resource {
            pid: Pid::from(pid),
            system: sysinfo::System::new(),
        }
    }

    pub fn pid_exists(&mut self) -> bool {
        self.system.refresh_processes();
        self.system.process(self.pid).is_some()
    }

    pub fn maybe_kill(&mut self, level: i32) -> bool {
        let signal = if level == 0 {
            sysinfo::Signal::Interrupt
        } else if level == 1 {
            sysinfo::Signal::Term
        } else {
            sysinfo::Signal::Kill
        };
        self.system.refresh_process(self.pid);
        match self.system.process(self.pid) {
            Some(p) => {
                info!("Attempting to send signal {} to PID {}", signal, self.pid);
                if p.kill_with(signal).is_some() {
                    info!("Signal: {} sent to PID: {}", signal, self.pid);
                    true
                } else {
                    warn!("Problem sending signal {}", signal);
                    false
                }
            }
            None => {
                warn!("PID {} not found while killing", self.pid);
                false
            }
        }
    }

    pub fn cpu_temp(&mut self) -> i32 {
        self.system.refresh_components_list();
        self.system.refresh_components();
        let mut max_temp = -1;
        for component in self.system.components() {
            if component.label().contains("core") {
                max_temp = std::cmp::max(max_temp, component.temperature() as i32);
            }
        }
        max_temp
    }

    pub fn cpu_util(&mut self) -> i32 {
        self.system.refresh_cpu();
        self.system.global_cpu_info().cpu_usage() as i32
    }

    pub fn sys_mem_percentage(&mut self) -> i32 {
        self.system.refresh_memory();
        (100.0 * self.system.used_memory() as f32 / self.system.total_memory() as f32) as i32
    }

    fn _parse_private_memory(&mut self, line: &str) -> i32 {
        if !line.contains(':') {
            0
        } else {
            // TODO: simplify this, converts (Private: 10 kb) -> (10)
            line.split_once(':').unwrap().1.trim().split_once(' ').unwrap().0.parse::<i32>().unwrap()
        }
    }

    pub fn proc_memory(&mut self) -> i32 {
        let data = fs::read_to_string(format!("/proc/{0}/smaps_rollup", self.pid))
                        .expect("{pid} process id does not exist!");
        let mut private = 0;
        for line in data.split('\n') {
            if line.starts_with("Private") {
                private += self._parse_private_memory(line);
            }
        }
        private
    }
}


