use log::*;

#[test]
fn debug_level_log() {
    buche::new()
        .module(module_path!())
        .timestamp(buche::Timestamp::Nanosecond)
        .verbosity(log::Level::Debug)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::Level::Debug, log::max_level())
}
