use file_rotate::{
    compression::Compression, suffix::AppendTimestamp, suffix::FileLimit, ContentLimit, FileRotate,
};
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

pub fn configure_logger() {
    CombinedLogger::init(vec![
        // stdio
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // file
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            FileRotate::new(
                "target/trace_log",
                AppendTimestamp::default(FileLimit::MaxFiles(10)),
                ContentLimit::Lines(100_000),
                Compression::None,
            ),
        ),
    ])
    .expect("failed to initialize logger");
}
