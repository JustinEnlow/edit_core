use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn move_up(){
    let text = Rope::from("idk\nsomething\nelse");
    
    // to shorter line
    assert_eq!(Selection::new(13, 13).move_up(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 9));
    assert_eq!(Selection::new(13, 14).move_up(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 9));
    
    // to longer line
    assert_eq!(Selection::new(18, 18).move_up(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(8, 8, 4));
    assert_eq!(Selection::new(18, 19).move_up(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(8, 9, 4));
    
    // with selection extended, collapses selection, then performs move
    assert_eq!(Selection::new(14, 14).move_up(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(14, 4).move_up(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(4, 14).move_up(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 9));
    assert_eq!(Selection::new(14, 4).move_up(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
}
#[test]
fn move_up_errors_if_on_topmost_line(){
    let text = Rope::from("idk\nsomething\nelse");
    assert!(Selection::new(0, 0).move_up(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).move_up(&text, CursorSemantics::Block).is_err());
}
