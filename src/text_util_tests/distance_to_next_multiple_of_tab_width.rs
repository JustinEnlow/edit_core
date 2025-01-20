use ropey::Rope;
use crate::document::TAB_WIDTH;
use crate::selection::{Selection, CursorSemantics};
use crate::text_util;

#[test] fn idk(){
    let mut tab = String::new();
    for _ in 0..TAB_WIDTH{
        tab.push(' ');
    }
    let text = Rope::from(format!("{}idk\n", tab));
    let selection = Selection::new(1, 1);
    let distance = text_util::distance_to_next_multiple_of_tab_width(selection, &text, CursorSemantics::Bar);
    assert!(distance == 3);
}
