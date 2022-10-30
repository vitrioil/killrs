use std::{fmt, fs};
use sysinfo::{ComponentExt, CpuExt, Pid, PidExt, ProcessExt, System, SystemExt};
use tracing::{info, warn};

/// Aggression defines what signal is received by the process
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Aggression {
    /// SIGINT
    Interrupt,
    /// SIGTERM
    Terminate,
    /// SIGKILL
    Kill,
}

impl fmt::Display for Aggression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Aggression {
    /// Escalate the signal from leaner to aggressive value
    fn escalate(&self) -> Self {
        match *self {
            Aggression::Interrupt => Aggression::Terminate,
            Aggression::Terminate => Aggression::Kill,
            Aggression::Kill => Aggression::Kill,
        }
    }

    /// Converts into sysinfo signal
    fn get_signal(&self) -> sysinfo::Signal {
        match *self {
            Aggression::Interrupt => sysinfo::Signal::Interrupt,
            Aggression::Terminate => sysinfo::Signal::Term,
            Aggression::Kill => sysinfo::Signal::Kill,
        }
    }
}

/// Resource options to monitor
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ResourceOptions {
    /// Process Memory
    ProcMem,
    /// System Memory
    SysMem,
    /// CPU Utilisation
    CpuUtil,
    /// CPU Temperature
    CpuTemp,
    /// Process Run Time
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
    aggression: Aggression,
    res_option: ResourceOptions,
    invert: i32,
    threshold: i32,
}

impl Resource {
    pub fn new(
        pid: i32,
        aggression: Aggression,
        res_option: ResourceOptions,
        invert: i32,
        threshold: i32,
    ) -> Resource {
        Resource {
            pid: Pid::from(pid),
            system: sysinfo::System::new(),
            aggression,
            res_option,
            invert,
            threshold,
        }
    }

    fn _parse_private_memory(&mut self, line: &str) -> i32 {
        if !line.contains(':') {
            0
        } else {
            // TODO: simplify this, converts (Private: 10 kb) -> (10)
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .split_once(' ')
                .unwrap()
                .0
                .parse::<i32>()
                .unwrap()
        }
    }

    fn proc_memory(&mut self) -> i32 {
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

    fn sys_mem_percentage(&mut self) -> i32 {
        self.system.refresh_memory();
        (100.0 * self.system.used_memory() as f32 / self.system.total_memory() as f32) as i32
    }

    fn cpu_util(&mut self) -> i32 {
        self.system.refresh_cpu();
        self.system.global_cpu_info().cpu_usage() as i32
    }

    fn cpu_temp(&mut self) -> i32 {
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

    fn run_time(&mut self) -> i32 {
        self.system.refresh_process(self.pid);
        match self.system.process(self.pid) {
            Some(p) => p.run_time() as i32,
            None => {
                warn!("PID {} not alive while getting run time", self.pid);
                0
            }
        }
    }

    fn _should_kill(&mut self) -> bool {
        match self.res_option {
            ResourceOptions::ProcMem => self.invert * (self.proc_memory() - self.threshold) > 0,
            ResourceOptions::SysMem => {
                self.invert * (self.sys_mem_percentage() - self.threshold) > 0
            }
            ResourceOptions::CpuUtil => self.invert * (self.cpu_util() - self.threshold) > 0,
            ResourceOptions::CpuTemp => self.invert * (self.cpu_temp() - self.threshold) > 0,
            ResourceOptions::RunTime => self.invert * (self.run_time() - self.threshold) > 0,
        }
    }

    /// Kill pid based on resource option and threshold
    /// Method will run until the process dies
    pub fn killrs(&mut self) {
        let mut wait = true;
        let timeout: u64 = 1;
        let mut signal_sent = false;
        while wait {
            if self._should_kill() {
                signal_sent = self.maybe_kill();
            }
            wait = signal_sent && self.pid_exists();
            if wait {
                std::thread::sleep(std::time::Duration::from_secs(timeout));
            }
        }
    }

    /// PID existence check
    pub fn pid_exists(&mut self) -> bool {
        self.system.refresh_all();
        self.system.process(self.pid).is_some()
    }

    fn _maybe_wait(&mut self, signal: sysinfo::Signal) {
        if signal == sysinfo::Signal::Kill {
            info!("Waiting for pid {}", self.pid);
            unsafe {
                // necessary to wait to ensure process died :)
                let mut status = 0 as libc::c_int;
                libc::waitpid(self.pid.as_u32() as i32, &mut status, 0);
            }
            info!("Wait complete for pid {}", self.pid);
        }
    }

    fn maybe_kill(&mut self) -> bool {
        let signal = self.aggression.get_signal();
        self.system.refresh_all();
        match self.system.process(self.pid) {
            Some(p) => {
                info!("Attempting to send signal {} to PID {}", signal, self.pid);
                if p.kill_with(signal).is_some() {
                    info!("Signal: {} sent to PID: {}", signal, self.pid);
                    self._maybe_wait(signal);
                    self.aggression = self.aggression.escalate();
                    info!("Aggression escalated to {}", self.aggression);
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
        let mut resource = Resource::new(
            p.id() as i32,
            Aggression::Kill,
            ResourceOptions::RunTime,
            1,
            10,
        );

        assert!(resource.pid_exists());
        assert!(resource.run_time() < 1);
        std::thread::sleep(std::time::Duration::from_millis(2000));
        assert!(resource.run_time() > 1);

        assert!(resource.proc_memory() != 0);
        assert!(resource.sys_mem_percentage() != 0);
        assert!(resource.cpu_util() != 0);
        assert!(resource.cpu_temp() != 0);

        // cleanup
        resource.maybe_kill();
    }

    #[test]
    #[should_panic(expected = "Error while reading private memory")]
    fn test_invalid_resource_proc_memory() {
        let mut resource = Resource::new(-1, Aggression::Kill, ResourceOptions::RunTime, 1, 10);

        resource.proc_memory();
    }

    #[test]
    fn test_invalid_resource() {
        let mut resource = Resource::new(-1, Aggression::Kill, ResourceOptions::RunTime, 1, 10);

        assert!(!resource.pid_exists());
        assert!(resource.run_time() == 0);

        // System resource will still be valid
        assert!(resource.sys_mem_percentage() != 0);
        assert!(resource.cpu_util() != 0);
        assert!(resource.cpu_temp() != 0);

        // cleanup
        resource.maybe_kill();
    }

    #[test]
    fn test_kill() {
        let p = _start_and_get_dummy_process(5);
        let mut resource = Resource::new(
            p.id() as i32,
            Aggression::Kill,
            ResourceOptions::RunTime,
            1,
            10,
        );

        assert!(resource.pid_exists());
        assert!(resource.maybe_kill());
        assert!(!resource.pid_exists());

        let mut invalid_resource =
            Resource::new(-1, Aggression::Interrupt, ResourceOptions::RunTime, 1, 10);
        assert!(!invalid_resource.maybe_kill());
        assert!(!invalid_resource.pid_exists());

        // cleanup
        resource.maybe_kill();
    }

    #[test]
    fn test_should_kill() {
        // create dependency
        let p = _start_and_get_dummy_process(5);
        std::thread::sleep(std::time::Duration::from_millis(1500));
        let mut resource_kill = Resource::new(
            p.id() as i32,
            Aggression::Kill,
            ResourceOptions::RunTime,
            1,
            0,
        );
        assert!(resource_kill._should_kill());

        let mut resource_dont_kill = Resource::new(
            p.id() as i32,
            Aggression::Kill,
            ResourceOptions::RunTime,
            1,
            10,
        );
        assert!(!resource_dont_kill._should_kill());

        // cleanup
        resource_dont_kill.maybe_kill();
        resource_kill.maybe_kill();
    }
}
