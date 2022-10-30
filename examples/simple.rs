// Run from root directory
fn main() {
    tracing_subscriber::fmt::init();
    // Some test process that handles SIGTERM!
    let p = std::process::Command::new("./examples/handler.py")
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let mut resource = killrs::resource::Resource::new(
        p.id() as i32,
        killrs::resource::Aggression::Terminate,
        killrs::resource::ResourceOptions::RunTime,
        1,
        1,
    );
    // Should kill after sending SIGKILL.
    resource.killrs();
}
