use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn move_line_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(3, 3).move_line_start(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(3, 4).move_line_start(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
}
#[test]
fn move_line_start_errors_if_already_at_line_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 0).move_line_start(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).move_line_start(&text, CursorSemantics::Block).is_err());
}
