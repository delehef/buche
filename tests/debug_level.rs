use log::*;

#[test]
fn debug_level() {
    buche::new()
        .module(module_path!())
        .timestamp(buche::Timestamp::Second)
        .verbosity(3)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::Level::Debug, log::max_level())
}
