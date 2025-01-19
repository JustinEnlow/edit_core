use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn extend_line_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 3).extend_line_start(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 4).extend_line_start(&text, CursorSemantics::Block).unwrap());   //special case  //if at end of line, sets anchor before newline char
}
#[test]
fn extend_line_start_errors_if_already_at_line_start(){
    let text = Rope::from("idk\n");
    assert!(Selection::new(0, 0).extend_home(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).extend_home(&text, CursorSemantics::Block).is_err());
}
