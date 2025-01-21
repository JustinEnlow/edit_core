use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn move_right_word_boundary(){
    let text = Rope::from("use std::error::Error;");
    assert_eq!(Selection::with_stored_line_position(2, 3, 2), Selection::new(0, 1).move_right_word_boundary(&text, CursorSemantics::Block).unwrap());
    assert_eq!(Selection::with_stored_line_position(3, 3, 3), Selection::new(0, 0).move_right_word_boundary(&text, CursorSemantics::Bar).unwrap());
}
#[test]
fn move_right_word_boundary_errors_if_at_doc_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(14, 15).move_right_word_boundary(&text, CursorSemantics::Block).is_err());
    assert!(Selection::new(14, 14).move_right_word_boundary(&text, CursorSemantics::Bar).is_err());
}
