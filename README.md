## KILLRS - Simple Resource Based Process Killer
### Use with caution
Killrs is a very simple threshold based process killer. Use it only for experimentation as killing a process can lead to unexpected results.

### Install
```bash
cargo install --git https://github.com/vitrioil/killrs
```

### How to use
Provide the pid, threshold and what resource to monitor.
```bash
killrs --help
```

Example:
To restrict the process running more than 100 seconds
```bash
killrs --pid <pid> --threshold 100 run-time
```

It will send SIGINT to begin with, but if the process still exists it will escalate it to SIGTERM and then SIGKILL.

Run example:
```bash
cargo run --example simple
```

### About
It uses sysinfo to gather system information.