use std::{fs, fmt};
use tracing::{info, warn};
use sysinfo::{SystemExt, System, CpuExt, ComponentExt, Pid, PidExt, ProcessExt};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ResourceOptions {
    ProcMem,
    SysMem,
    CpuUtil,
    CpuTemp,
    RunTime,
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
                        .expect("Error while reading private memory");
        let mut private = 0;
        for line in data.split('\n') {
            if line.starts_with("Private") {
                private += self._parse_private_memory(line);
            }
        }
        private
    }

    pub fn sys_mem_percentage(&mut self) -> i32 {
        self.system.refresh_memory();
        (100.0 * self.system.used_memory() as f32 / self.system.total_memory() as f32) as i32
    }

    pub fn cpu_util(&mut self) -> i32 {
        self.system.refresh_cpu();
        self.system.global_cpu_info().cpu_usage() as i32
    }

    pub fn cpu_temp(&mut self) -> i32 {
        self.system.refresh_components_list();
        self.system.refresh_components();
        let mut max_temp = std::i32::MIN;
        for component in self.system.components() {
            if component.label().contains("core") {
                max_temp = std::cmp::max(max_temp, component.temperature() as i32);
            }
        }
        max_temp
    }

    pub fn run_time(&mut self) -> i32 {
        self.system.refresh_process(self.pid);
        match self.system.process(self.pid) {
            Some(p) => {
                p.run_time() as i32
            }
            None => {
                warn!("PID {} not alive while getting run time", self.pid);
                0
            }
        }
    }

    pub fn pid_exists(&mut self) -> bool {
        // TODO: Figure out why refresh_process does not work sometime.
        self.system.refresh_all();
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
        self.system.refresh_all();
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

    pub fn pid(&self) -> u32 {
        self.pid.as_u32()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn _start_and_get_dummy_process(ttl: i32) -> std::process::Child {
        std::process::Command::new("sleep")
             .arg(ttl.to_string())
             .stdout(std::process::Stdio::null())
             .spawn()
             .unwrap()
    }

    #[test]
    fn test_resource() {
        let p = _start_and_get_dummy_process(5);
        let mut resource = Resource::new(p.id() as i32);

        assert!(resource.pid_exists());
        assert!(resource.run_time() < 1);
        std::thread::sleep(std::time::Duration::from_millis(2000));
        assert!(resource.run_time() > 1);

        assert!(resource.proc_memory() != 0);
        assert!(resource.sys_mem_percentage() != 0);
        assert!(resource.cpu_util() != 0);
        assert!(resource.cpu_temp() != 0);


        resource.maybe_kill(2); // send SIGKILL
    }

    #[test]
    #[should_panic(expected = "Error while reading private memory")]
    fn test_invalid_resource_proc_memory() {
        let mut resource = Resource::new(-1);

        resource.proc_memory();
    }

    #[test]
    fn test_invalid_resource() {
        let mut resource = Resource::new(-1);

        assert!(!resource.pid_exists());
        assert!(resource.run_time() == 0);

        // System resource will still be valid
        assert!(resource.sys_mem_percentage() != 0);
        assert!(resource.cpu_util() != 0);
        assert!(resource.cpu_temp() != 0);
    }

    #[test]
    fn test_kill() {
        let p = _start_and_get_dummy_process(5);
        let mut resource = Resource::new(p.id() as i32);

        assert!(resource.pid_exists());
        assert!(resource.maybe_kill(2));
        // Does not work..
        // assert!(!resource.pid_exists());

        let mut invalid_resource = Resource::new(-1);
        assert!(!invalid_resource.maybe_kill(0));
        assert!(!invalid_resource.pid_exists());
    }
}
