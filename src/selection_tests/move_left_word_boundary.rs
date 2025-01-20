use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn move_left_word_boundary(){
    let text = Rope::from("use std::error::Error;");
    assert_eq!(Selection::with_stored_line_position(0, 1, 0), Selection::new(3, 4).move_left_word_boundary(&text, CursorSemantics::Block).unwrap());
}
#[test]
fn move_left_word_boundary_errors_if_at_doc_start(){
    assert!(false);
}
