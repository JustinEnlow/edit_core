use ropey::Rope;
use crate::document::TAB_WIDTH;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};
use crate::text_util;

#[test] fn idk(){
    let mut tab = String::new();
    for _ in 0..TAB_WIDTH{
        tab.push(' ');
    }
    let text = Rope::from(format!("{}idk\n", tab));
    let selection = Selection::new(Range::new(1, 1), Direction::Forward);
    let distance = text_util::distance_to_next_multiple_of_tab_width(&selection, &text, CursorSemantics::Bar);
    assert!(distance == 3);
}
