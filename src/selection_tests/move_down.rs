use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn move_down(){
    let text = Rope::from("idk\nsomething\nelse");
    
    // to longer line
    assert_eq!(Selection::new(3, 3).move_down(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(7, 7, 3));
    assert_eq!(Selection::new(3, 4).move_down(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(7, 8, 3));
    
    // to shorter line
    assert_eq!(Selection::new(13, 13).move_down(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(18, 18, 9));
    assert_eq!(Selection::new(13, 14).move_down(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(18, 19, 9));
    
    // with selection extended, collapses selection, then performs move
    assert_eq!(Selection::new(0, 4).move_down(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(14, 14, 0));
    assert_eq!(Selection::new(4, 0).move_down(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    //[i d k \n]s o m e \n s h i t \n
    // i d k \n s o m[e]\n s h i t \n
    assert_eq!(Selection::new(0, 4).move_down(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(7, 8, 3));
    assert_eq!(Selection::new(4, 0).move_down(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
}
#[test]
fn move_down_errors_if_on_bottommost_line(){
    let text = Rope::from("idk\nsomething\nelse");
    assert!(Selection::new(18, 18).move_down(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(18, 19).move_down(&text, CursorSemantics::Block).is_err());
}
