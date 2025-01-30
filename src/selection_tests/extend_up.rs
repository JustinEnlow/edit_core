use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn extend_up(){
    let text = Rope::from("idk\nsomething\nelse");
    // idk test //i think this fails because of how `with_stored_line_position` is currently implemented
    assert_eq!(Selection::with_stored_line_position(5, 1, 0), Selection::new(4, 5).extend_up(&text, CursorSemantics::Block).unwrap());
    
    // to shorter line
    assert_eq!(Selection::with_stored_line_position(13, 3, 9), Selection::new(13, 13).extend_up(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(13, 3, 9), Selection::new(13, 14).extend_up(&text, CursorSemantics::Block).unwrap()); //if at end of line, sets anchor before newline char
    
    // to longer line
    assert_eq!(Selection::with_stored_line_position(18, 8, 4), Selection::new(18, 18).extend_up(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(18, 8, 4), Selection::new(18, 19).extend_up(&text, CursorSemantics::Block).unwrap()); //idk\nsomething\nelse[: ]   //idk\nsome:]thing\nelse[
}
#[test]
fn extend_up_errors_if_on_topmost_line(){
    let text = Rope::from("idk\nsomething\nelse");
    assert!(Selection::new(0, 0).extend_up(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).extend_up(&text, CursorSemantics::Block).is_err());
}
