mod args;
mod metadata;
mod util;
mod calc;
mod format;
mod sort;

use tokio;
use crate::args::app;
use std::path::{PathBuf};
use std::str::FromStr;
use crate::format::{RoundFactor, FormatUnit, format_record};
use crate::sort::SortKey;


#[tokio::main]
async fn main() {
    let app = app();
    let matches = app.get_matches();

    let path = PathBuf::from(matches.value_of("path").unwrap_or("."));
    let mut depth = u16::from_str(matches.value_of("depth").unwrap_or("1"))
        .expect("Invalid depth parameter specified. Expected numerical value.");
    let places = RoundFactor::parse(i8::from_str(matches.value_of("places").unwrap_or("0"))
        .expect("Invalid round parameter specified. Expected numerical value."));
    let unit = FormatUnit::parse(matches.value_of("unit").unwrap_or("auto"))
        .unwrap_or(FormatUnit::Kilobytes);

    let mut files = matches.is_present("files");
    let empty = matches.is_present("empty");
    let tree = matches.is_present("tree");

    let sort = SortKey::parse(matches.value_of("sort").unwrap_or("order"))
        .expect("Invalid sort parameter specified. Expected valid sort.");
    let invert = matches.is_present("invert");

    if tree {
        depth = u16::MAX;
        files = true;
    }

    let calc = calc::handle_folder(path, depth).await.expect("Failed to calculate");

    format_record(calc, 0, false, &unit, &places, &files, &empty, &sort, &invert);
}
