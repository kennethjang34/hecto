#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

mod document;
mod editor;
mod filetype;
mod highlighting;
mod row;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use editor::SearchDirection;
pub use env_logger;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;
pub use log::{debug, info, trace, warn};
pub use row::Row;
pub use simple_log::LogConfigBuilder;
//use std::time::SystemTime;
use chrono::Local;
pub use terminal::Terminal;

fn main() {
    let config = LogConfigBuilder::builder()
        .path(format!(
            "../hecto/log/{}.log",
            Local::now().format("%Y-%m-%d_%H-%M-%S")
        ))
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level("debug")
        .output_file()
        .output_console()
        .build();

    if let Ok(_) = simple_log::new(config) {
    } else {
        env_logger::init();
    }
    let mut editor = Editor::default();
    editor.run();
}
