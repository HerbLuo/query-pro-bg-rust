use log::{LevelFilter};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use std::borrow::Borrow;

const _1M: u64 = 1024 * 1024;

const LOG_FILE_NAME: &str = "query-pro-server";

pub fn init() {
    let file_path = "logs/".to_owned() + LOG_FILE_NAME + ".log";

    // Logging to stderr
    let stderr = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} {d} {M} line {L} - {m}\n")))
        .target(Target::Stderr)
        .build();

    // Logging to log file.
    let logfile = RollingFileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} {d} {M} {L} - {m}\n")))
        .build(file_path, Box::new(CompoundPolicy::new(
            Box::new(SizeTrigger::new(5 * _1M)),
            Box::new(FixedWindowRoller::builder()
                .build(("logs/".to_owned() + LOG_FILE_NAME + ".{}.log").borrow(), 20)
                .unwrap())
        )))
        .unwrap();

    // Log Trace output to file, and the user-specified level to stderr.
    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                .build("logfile", Box::new(logfile))
        )
        .appender(
            Appender::builder()
                .build("stderr", Box::new(stderr)),
        )
        .build(
            if cfg!(debug_assertions) {
                Root::builder()
                    .appender("logfile")
                    .appender("stderr")
                    .build(LevelFilter::Info)
//                    .build(LevelFilter::Trace)
            } else {
                Root::builder()
                    .appender("logfile")
                    .build(LevelFilter::Info)
            }
        )
        .unwrap();

    // Use this to change log levels at runtime.
    let _handle = log4rs::init_config(config).unwrap();
}
