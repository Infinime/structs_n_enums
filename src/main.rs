use chrono::{DateTime, Local};
use clippers;
fn main() {
    let now: DateTime<Local> = Local::now();
    let dt = now.format("(%Y-%m-%d, %I:%M%p)").to_string();
    let mut clipboard = clippers::Clipboard::get();
    clipboard.write_text(dt).unwrap();
}
