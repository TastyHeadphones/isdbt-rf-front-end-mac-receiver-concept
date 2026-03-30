use std::path::PathBuf;

use isdbt_shared::config::{PlatformConfig, RuntimeMode};

#[test]
fn loads_default_config() {
    let path = PathBuf::from("config/default.toml");
    let cfg = PlatformConfig::load_from_file(path).expect("config should load");

    match cfg.mode {
        RuntimeMode::LabDemo => {}
        _ => panic!("expected lab_demo mode"),
    }

    assert_eq!(cfg.http.port, 8088);
    assert!(cfg.lab.simulate_devices);
}
