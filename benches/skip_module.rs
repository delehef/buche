#![feature(test)]

mod common;
mod util;

use std::sync;

static INIT_LOGGER: sync::Once = sync::ONCE_INIT;

fn init_logger() {
    INIT_LOGGER.call_once(|| {
        let mut other_modules = Vec::new();
        for i in 0..10_000 {
            other_modules.push(format!("other::mod{}", i));
        }
        buche::Buche::new()
            .timestamp(buche::Timestamp::Second)
            .verbosity(10)
            .modules(other_modules)
            .init()
            .unwrap();
    });
}
