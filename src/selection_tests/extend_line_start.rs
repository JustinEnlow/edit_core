use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test]
fn extend_line_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    //assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 3).extend_line_start(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0), Selection::new(Range::new(3, 3), Direction::Forward).extend_line_start(&text, CursorSemantics::Bar).unwrap());
    //assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 4).extend_line_start(&text, CursorSemantics::Block).unwrap());   //special case  //if at end of line, sets anchor before newline char
    assert_eq!(Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0), Selection::new(Range::new(3, 4), Direction::Forward).extend_line_start(&text, CursorSemantics::Block).unwrap());   //special case  //if at end of line, sets anchor before newline char
}
#[test]
fn extend_line_start_errors_if_already_at_line_start(){
    let text = Rope::from("idk\n");
    //assert!(Selection::new(0, 0).extend_home(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(Range::new(0, 0), Direction::Forward).extend_home(&text, CursorSemantics::Bar).is_err());
    //assert!(Selection::new(0, 1).extend_home(&text, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(0, 1), Direction::Forward).extend_home(&text, CursorSemantics::Block).is_err());
}
