#![feature(test)]

mod common;
mod util;

use std::sync;

static INIT_LOGGER: sync::Once = sync::ONCE_INIT;

fn init_logger() {
    INIT_LOGGER.call_once(|| {
        buche::Buche::new()
            .timestamp(buche::Timestamp::Second)
            .verbosity(10)
            .quiet(true)
            .module(module_path!())
            .init()
            .unwrap();
    });
}
