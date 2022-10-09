#[test]
fn test_monitor() {
    let mut resource = killrs::resource::Resource::new(-1);

    let before = std::time::Instant::now();
    killrs::monitor::monitor(&mut resource, 2, killrs::resource::ResourceOptions::RunTime, 1, 5);

    assert!(before.elapsed() < std::time::Duration::from_millis(1100));
}
