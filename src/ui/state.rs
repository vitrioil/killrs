

#[derive(Default)]
pub struct SysResource {
    /// System Memory
    pub sys_mem: i32,

    /// CPU Utilization
    pub cpu_util: i32,

    /// CPU Temperature
    pub cpu_temp: i32
}


/// UI State
#[derive(Default)]
pub struct State {
    pub sys_resource: SysResource
}

impl State {
    pub fn refresh_all() {

    }
}
