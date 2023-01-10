use log::*;

#[test]
fn trace_level() {
    buche::new()
        .module(module_path!())
        .verbosity(4)
        .init()
        .unwrap();

    error!("error msg");
    warn!("warning msg");
    info!("info msg");
    debug!("debug msg");
    trace!("trace msg");

    assert_eq!(log::Level::Trace, log::max_level())
}
