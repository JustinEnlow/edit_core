use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn select_all(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(Selection::with_stored_line_position(0, 14, 0), Selection::new(0, 0).select_all(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 15, 0), Selection::new(0, 1).select_all(&text, CursorSemantics::Block).unwrap());
}
#[test]
fn select_all_errors_if_already_all_selected(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 14).select_all(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 15).select_all(&text, CursorSemantics::Block).is_err());
}
