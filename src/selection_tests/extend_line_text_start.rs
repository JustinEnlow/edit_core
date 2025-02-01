use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test]
fn extend_line_text_start(){
    let text = Rope::from("  idk\n");
    //assert_eq!(Selection::with_stored_line_position(0, 2, 2), Selection::new(0, 0).extend_line_text_start(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(Range::new(0, 2), Direction::Forward, 2), Selection::new(Range::new(0, 0), Direction::Forward).extend_line_text_start(&text, CursorSemantics::Bar).unwrap());
    //assert_eq!(Selection::with_stored_line_position(0, 3, 2), Selection::new(0, 1).extend_line_text_start(&text, CursorSemantics::Block).unwrap());
    assert_eq!(Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 2), Selection::new(Range::new(0, 1), Direction::Forward).extend_line_text_start(&text, CursorSemantics::Block).unwrap());
}
#[test]
fn extend_line_text_start_errors_if_already_at_text_start(){
    let text = Rope::from("    idk\n");
    //assert!(Selection::new(4, 4).extend_line_text_start(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(Range::new(4, 4), Direction::Forward).extend_line_text_start(&text, CursorSemantics::Bar).is_err());
    //assert!(Selection::new(4, 5).extend_line_text_start(&text, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(4, 5), Direction::Forward).extend_line_text_start(&text, CursorSemantics::Block).is_err());
}
