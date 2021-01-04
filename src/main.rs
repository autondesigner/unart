#![allow(warnings)]

mod address;
mod address_book;
mod automaton;
mod automaton_viewer;
mod buildable;
mod cell;
mod color;
mod direction;
mod image_torus;
mod rule_key;
mod tape;
mod torus;
mod track;
mod tracker_viewer;
mod util;

use crate::tracker_viewer::*;
use std::path::Path;

fn main() {
    let mut viewer = TrackerViewer::new((512 + 256), 12);
    let path = Path::new("a0");
    viewer.render(&path);
}
