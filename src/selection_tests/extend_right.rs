use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn extend_right(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // normal use
    assert_eq!(Selection::new(0, 0).extend_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 1, 1));
    assert_eq!(Selection::new(0, 1).extend_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 2, 1));
    assert_eq!(Selection::new(1, 0).extend_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 2, 1));   //so backward cursor behaves the same as forward...
    
    // resets stored line position after new line
    assert_eq!(Selection::new(3, 3).extend_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 4, 0));
    assert_eq!(Selection::new(3, 4).extend_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 5, 0));
    
    // previously extended
    assert_eq!(Selection::new(0, 3).extend_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(3, 0).extend_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 1, 1));
    assert_eq!(Selection::new(0, 3).extend_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 4, 3));
    assert_eq!(Selection::new(3, 0).extend_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 1, 1));
}
#[test]
fn extend_right_errors_if_at_doc_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(14, 14).extend_right(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(14, 15).extend_right(&text, CursorSemantics::Block).is_err());
}
