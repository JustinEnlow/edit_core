use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn extend_line_text_start(){
    let text = Rope::from("  idk\n");
    assert_eq!(Selection::with_stored_line_position(0, 2, 2), Selection::new(0, 0).extend_line_text_start(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 3, 2), Selection::new(0, 1).extend_line_text_start(&text, CursorSemantics::Block).unwrap());
}
#[test]
fn extend_line_text_start_errors_if_already_at_text_start(){
    let text = Rope::from("    idk\n");
    assert!(Selection::new(4, 4).extend_line_text_start(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(4, 5).extend_line_text_start(&text, CursorSemantics::Block).is_err());
}
