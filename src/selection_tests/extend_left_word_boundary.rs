use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test] fn extend_left_word_boundary(){
    let text = Rope::from("use std::error::Error;");
    //assert_eq!(Selection::with_stored_line_position(4, 0, 0), Selection::new(3, 4).extend_left_word_boundary(&text, CursorSemantics::Block).unwrap());
    assert_eq!(Selection::with_stored_line_position(Range::new(0, 4), Direction::Backward, 0), Selection::new(Range::new(3, 4), Direction::Forward).extend_left_word_boundary(&text, CursorSemantics::Block).unwrap());
    //assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 3).extend_left_word_boundary(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0), Selection::new(Range::new(3, 3), Direction::Forward).extend_left_word_boundary(&text, CursorSemantics::Bar).unwrap());
}

#[test] fn extend_left_word_boundary_error(){
    let text = Rope::from("idk\nsome\nshit\n");
    //assert!(Selection::new(0, 1).extend_left_word_boundary(&text, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(0, 1), Direction::Forward).extend_left_word_boundary(&text, CursorSemantics::Block).is_err());
    //assert!(Selection::new(0, 0).extend_left_word_boundary(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(Range::new(0, 0), Direction::Forward).extend_left_word_boundary(&text, CursorSemantics::Bar).is_err());
}
