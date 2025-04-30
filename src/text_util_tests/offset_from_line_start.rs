use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction, CursorSemantics};
use crate::text_util;

#[test] fn idk(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(Range::new(2, 2), Direction::Forward);
    let selection = Selection::new_from_range(Range::new(2, 2), Direction::Forward, &text, CursorSemantics::Bar);
    assert!(text_util::offset_from_line_start(selection.head(), &text) == 2);
}
