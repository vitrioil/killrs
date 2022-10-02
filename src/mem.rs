use std::{fs, process::Command};

fn get_memory_value(line: &str) -> i32 {
    if !line.contains(':') {
        0
    } else {
        // TODO: simplify this, converts (Private: 10 kb) -> (10)
        line.split_once(':').unwrap().1.trim().split_once(' ').unwrap().0.parse::<i32>().unwrap()
    }
}

pub fn read_system_memory_percentage() -> i32 {
    match String::from_utf8(Command::new("sh")
                     .arg("-c")
                     .arg("free | grep Mem | awk '{print int( $3/$2 * 100.0 ) }'")
                     .output().expect("Failed to execute kill").stdout) {
        Ok(output) => output.replace('\n', "").parse::<i32>().unwrap(),
        Err(_) => 0
    }
}

pub fn read_private_memory(pid: i32) -> i32 {
    let data = fs::read_to_string(format!("/proc/{pid}/smaps_rollup"))
                    .expect("{pid} process id does not exist!");
    let mut private = 0;
    for line in data.split('\n') {
        if line.starts_with("Private") {
            private += get_memory_value(line);
        }
    }
    private
}
