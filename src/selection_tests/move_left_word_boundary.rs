use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test]
fn move_left_word_boundary(){
    let text = Rope::from("use std::error::Error;");
    //assert_eq!(Selection::with_stored_line_position(0, 1, 0), Selection::new(3, 4).move_left_word_boundary(&text, CursorSemantics::Block).unwrap());
    assert_eq!(Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0), Selection::new(Range::new(3, 4), Direction::Forward).move_left_word_boundary(&text, CursorSemantics::Block).unwrap());
    //assert_eq!(Selection::with_stored_line_position(0, 0, 0), Selection::new(3, 3).move_left_word_boundary(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0), Selection::new(Range::new(3, 3), Direction::Forward).move_left_word_boundary(&text, CursorSemantics::Bar).unwrap());
}
#[test]
fn move_left_word_boundary_errors_if_at_doc_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    //assert!(Selection::new(0, 1).move_left_word_boundary(&text, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(0, 1), Direction::Forward).move_left_word_boundary(&text, CursorSemantics::Block).is_err());
    //assert!(Selection::new(0, 0).move_left_word_boundary(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(Range::new(0, 0), Direction::Forward).move_left_word_boundary(&text, CursorSemantics::Bar).is_err());
}
