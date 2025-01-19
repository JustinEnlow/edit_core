use ropey::Rope;
use crate::selection::{Selection, CursorSemantics, Movement};

#[test]
fn set_from_line_number(){
    let text = Rope::from("idk\nsomething\nelse\n");
    
    // normal use
    assert_eq!(Selection::new(0, 0).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(14, 14, 0));
    assert_eq!(Selection::new(0, 1).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(14, 15, 0));
    assert_eq!(Selection::new(0, 0).set_from_line_number(2, &text, Movement::Extend, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 14, 0));
    assert_eq!(Selection::new(0, 1).set_from_line_number(2, &text, Movement::Extend, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 15, 0));
    
    // restricts cursor to line end when stored_line_position > line width
    assert_eq!(Selection::new(13, 13).set_from_line_number(0, &text, Movement::Move, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 9));
    assert_eq!(Selection::new(13, 14).set_from_line_number(0, &text, Movement::Move, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 9));
    assert_eq!(Selection::new(13, 13).set_from_line_number(0, &text, Movement::Extend, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(13, 3, 9));
    assert_eq!(Selection::new(13, 14).set_from_line_number(0, &text, Movement::Extend, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(13, 3, 9));    //if at end of line, sets anchor before newline char
    
    //from end of text
    assert_eq!(Selection::new(19, 19).set_from_line_number(1, &text, Movement::Move, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(19, 20).set_from_line_number(1, &text, Movement::Move, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
    assert_eq!(Selection::new(19, 19).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(14, 14, 0));
    assert_eq!(Selection::new(19, 20).set_from_line_number(2, &text, Movement::Move, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(14, 15, 0));
}
#[test]
fn set_from_line_number_should_error_when_goal_line_number_greater_than_len_lines(){
    let text = Rope::from("idk\nsomething\nelse\n");    //num lines 4
    assert!(Selection::new(0, 0).set_from_line_number(5, &text, Movement::Move, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).set_from_line_number(5, &text, Movement::Move, CursorSemantics::Block).is_err());
}
