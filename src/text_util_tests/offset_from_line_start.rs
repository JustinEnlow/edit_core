use ropey::Rope;
use crate::selection::Selection;
use crate::text_util;

#[test] fn idk(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(2, 2);
    assert!(text_util::offset_from_line_start(selection.head(), &text) == 2);
}
