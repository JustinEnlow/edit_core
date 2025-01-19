use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn move_line_text_end(){
    let text = Rope::from("idk\nsomething\nelse\n");
    
    assert_eq!(Selection::new(0, 0).move_line_text_end(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 3));
    assert_eq!(Selection::new(0, 1).move_line_text_end(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));
    
    // with selection extended, collapse and move
    assert_eq!(Selection::new(0, 2).move_line_text_end(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 3));
    assert_eq!(Selection::new(2, 0).move_line_text_end(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 3));
    assert_eq!(Selection::new(0, 2).move_line_text_end(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));
    assert_eq!(Selection::new(2, 0).move_line_text_end(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));
}
#[test]
fn move_line_text_end_errors_if_already_at_line_text_end(){
    let text = Rope::from("idk\nsomething\nelse\n");
    assert!(Selection::new(3, 3).extend_line_text_end(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(2, 3).extend_line_text_end(&text, CursorSemantics::Block).is_err());
}
