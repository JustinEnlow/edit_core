use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test] fn extend_left_word_boundary(){
    let text = Rope::from("use std::error::Error;");
    assert_eq!(Selection::with_stored_line_position(4, 0, 0), Selection::new(3, 4).extend_left_word_boundary(&text, CursorSemantics::Block).unwrap());
    assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 3).extend_left_word_boundary(&text, CursorSemantics::Bar).unwrap());
}

#[test] fn extend_left_word_boundary_error(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 1).extend_left_word_boundary(&text, CursorSemantics::Block).is_err());
    assert!(Selection::new(0, 0).extend_left_word_boundary(&text, CursorSemantics::Bar).is_err());
}
