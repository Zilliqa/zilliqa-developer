mod app;
mod bytecode_view;
mod dropdown;
mod examples;
mod machine_view;
mod state;
mod vm_remote;
mod vm_remote_layout;
mod vm_remote_state;

mod logger;

use app::App;
use log::{Log, Metadata, Record};
use yewdux::prelude::Dispatch;

use crate::logger::{LoggerMessage, LoggerState};

struct CaptureLogger {}

impl CaptureLogger {
    fn new() -> Self {
        Self {}
    }
}

impl Log for CaptureLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // self.delegate.enabled(metadata)
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let dispatch = Dispatch::<LoggerState>::new();
            dispatch.apply(LoggerMessage::Log {
                level: record.level().to_string(),
                value: record.args().to_string(),
            });
        }
    }

    fn flush(&self) {}
}

// Later, you'd set the logger as:
fn setup_logger() {
    let logger = Box::new(CaptureLogger::new());
    log::set_boxed_logger(logger).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}

fn main() {
    setup_logger();

    // wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
