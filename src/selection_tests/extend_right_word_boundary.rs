use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test] fn extend_right_word_boundary(){
    let text = Rope::from("use std::error::Error;");
    assert_eq!(Selection::with_stored_line_position(0, 3, 2), Selection::new(0, 1).extend_right_word_boundary(&text, CursorSemantics::Block).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 3, 3), Selection::new(0, 0).extend_right_word_boundary(&text, CursorSemantics::Bar).unwrap());
}

#[test] fn extend_right_word_boundary_error(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(14, 15).extend_right_word_boundary(&text, CursorSemantics::Block).is_err());
    assert!(Selection::new(14, 14).extend_right_word_boundary(&text, CursorSemantics::Bar).is_err());
}
