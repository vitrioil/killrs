// this process can't be killed when spawned
fn _start_and_get_dummy_process(ttl: i32) -> std::process::Child {
    std::process::Command::new("sleep")
        .arg(ttl.to_string())
        .spawn()
        .unwrap()
}

#[test]
fn test_sigkill_time() {
    let mut p = _start_and_get_dummy_process(2);

    let mut resource = killrs::resource::Resource::new(
        p.id() as i32,
        killrs::resource::Aggression::Kill,
        killrs::resource::ResourceOptions::SysMem,
        1,
        1,
    );

    let before = std::time::Instant::now();
    resource.killrs();

    assert!(before.elapsed() < std::time::Duration::from_millis(1500));

    // clean up
    let _ = p.wait();
}
