use std::io::stderr;
use std::io::Write;
use std::error::Error;
use android_logger::{Config};
use log::{error, LevelFilter, trace};


pub fn init_once() {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Debug)
            .with_tag("wwww")
            // .with_filter(FilterBuilder::new()
            //     .parse("debug,hello::crate=error")
            //     .build())
    );
}

pub fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "{}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "Caused by: {}", source);
        err = source;
    }
}