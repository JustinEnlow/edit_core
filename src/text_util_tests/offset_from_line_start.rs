use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction};
use crate::text_util;

#[test] fn idk(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(Range::new(2, 2), Direction::Forward);
    assert!(text_util::offset_from_line_start(selection.head(), &text) == 2);
}
