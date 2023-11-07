use log::{info, MetadataBuilder, Record, RecordBuilder, trace, warn};


use log::LevelFilter;
use android_logger::Config;

pub fn init_once() {
    android_logger::init_once(
        Config::default().with_max_level(LevelFilter::Trace),
    );
}

pub fn test_record_builder() {
    use log::{MetadataBuilder, RecordBuilder};
    let target = "myApp";
    let metadata = MetadataBuilder::new().target(target).build();
    let fmt_args = format_args!("hello");
    let record_test = RecordBuilder::new()
        .args(fmt_args)
        .metadata(metadata)
        .module_path(Some("foo"))
        .file(Some("bar"))
        .line(Some(30))
        .build();
   android_logger::log(&record_test);
}