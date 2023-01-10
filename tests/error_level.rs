use log::*;

#[test]
fn error_level() {
    buche::new()
        .module(module_path!())
        .timestamp(buche::Timestamp::Millisecond)
        .verbosity(0)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::Level::Error, log::max_level())
}
