use log::*;

#[test]
fn quiet_trace_level() {
    buche::new()
        .module(module_path!())
        .timestamp(buche::Timestamp::Second)
        .verbosity(4)
        .quiet(true)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::LevelFilter::Off, log::max_level())
}
