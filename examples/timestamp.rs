use clap::{crate_version, App, Arg};
use log::*;

fn main() {
    let m = App::new("buche example")
        .version(crate_version!())
        .arg(
            Arg::with_name("verbosity")
                .short('v')
                .multiple(true)
                .help("Increase message verbosity"),
        )
        .arg(
            Arg::with_name("quiet")
                .short('q')
                .help("Silence all output"),
        )
        .arg(
            Arg::with_name("timestamp")
                .short('t')
                .help("prepend log lines with a timestamp")
                .takes_value(true)
                .possible_values(&["none", "sec", "ms", "us", "ns"]),
        )
        .get_matches();

    let verbose = m.occurrences_of("verbosity") as usize;
    let quiet = m.is_present("quiet");
    let ts = match m.value_of("timestamp") {
        Some("ns") => buche::Timestamp::Nanosecond,
        Some("ms") => buche::Timestamp::Millisecond,
        Some("us") => buche::Timestamp::Microsecond,
        Some("sec") => buche::Timestamp::Second,
        Some("none") | None => buche::Timestamp::Off,
        Some(_) => clap::Error::raw(
            clap::ErrorKind::InvalidValue,
            "invalid value for 'timestamp'",
        )
        .exit(),
    };

    buche::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        .timestamp(ts)
        .init()
        .unwrap();
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}
