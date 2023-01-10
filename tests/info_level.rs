use log::*;

#[test]
fn info_level() {
    buche::new()
        .module(module_path!())
        .timestamp(buche::Timestamp::Microsecond)
        .verbosity(2)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::Level::Info, log::max_level())
}
