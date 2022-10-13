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
    
    let mut resource = killrs::resource::Resource::new(p.id() as i32);

    let before = std::time::Instant::now();
    killrs::monitor::monitor(&mut resource, 2, killrs::resource::ResourceOptions::SysMem, 1, 1);

    assert!(before.elapsed() < std::time::Duration::from_millis(1500));

    // clean up
    let _ = p.wait();
}

#[test]
fn test_sigterm_time() {
    let mut p = _start_and_get_dummy_process(3);
    
    let mut resource = killrs::resource::Resource::new(p.id() as i32);

    let before = std::time::Instant::now();
    // should sleep twice, then escalate to sigkill and stop
    killrs::monitor::monitor(&mut resource, 1, killrs::resource::ResourceOptions::SysMem, 1, 1);

    assert!(before.elapsed() > std::time::Duration::from_millis(2000));
    assert!(before.elapsed() < std::time::Duration::from_millis(3000));

    // clean up
    let _ = p.wait();
}

#[test]
fn test_sigint_time() {
    let mut p = _start_and_get_dummy_process(7);
    
    let mut resource = killrs::resource::Resource::new(p.id() as i32);

    let before = std::time::Instant::now();
    // should sleep 4 times, then escalate to sigkill and stop
    killrs::monitor::monitor(&mut resource, 0, killrs::resource::ResourceOptions::SysMem, 1, 1);

    assert!(before.elapsed() > std::time::Duration::from_millis(6000));
    assert!(before.elapsed() < std::time::Duration::from_millis(7000));

    // clean up
    let _ = p.wait();
}
