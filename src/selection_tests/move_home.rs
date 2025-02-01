use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test]
fn move_home(){
    let text = Rope::from("    idk\n");
    
    // moves to text start when cursor past text start
    //assert_eq!(Selection::new(6, 6).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 4));
    assert_eq!(Selection::new(Range::new(6, 6), Direction::Forward).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 4));
    //assert_eq!(Selection::new(6, 7).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 4));
    assert_eq!(Selection::new(Range::new(6, 7), Direction::Forward).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 4));
    
    // moves to line start when cursor at text start
    //assert_eq!(Selection::new(4, 4).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(Range::new(4, 4), Direction::Forward).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0));
    //assert_eq!(Selection::new(4, 5).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(Range::new(4, 5), Direction::Forward).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0));
    
    // moves to text start when cursor before text start
    //assert_eq!(Selection::new(1, 1).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 4));
    assert_eq!(Selection::new(Range::new(1, 1), Direction::Forward).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 4));
    //assert_eq!(Selection::new(1, 2).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 4));
    assert_eq!(Selection::new(Range::new(1, 2), Direction::Forward).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 4));
    
    // with selection extended, collapse and move
    //assert_eq!(Selection::new(0, 5).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 4));
    assert_eq!(Selection::new(Range::new(0, 5), Direction::Forward).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 4));
    //assert_eq!(Selection::new(0, 3).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 4));
    assert_eq!(Selection::new(Range::new(0, 3), Direction::Forward).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 4));
    //assert_eq!(Selection::new(0, 4).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(Range::new(0, 4), Direction::Forward).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0));
    //assert_eq!(Selection::new(5, 0).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 4));
    assert_eq!(Selection::new(Range::new(0, 5), Direction::Backward).move_home(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 4));
    //assert_eq!(Selection::new(0, 6).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 4));
    assert_eq!(Selection::new(Range::new(0, 6), Direction::Forward).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 4));
    //assert_eq!(Selection::new(0, 4).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 4));
    assert_eq!(Selection::new(Range::new(0, 4), Direction::Forward).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 4));
    //assert_eq!(Selection::new(0, 5).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(Range::new(0, 5), Direction::Forward).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0));
    //assert_eq!(Selection::new(5, 0).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 4));
    assert_eq!(Selection::new(Range::new(0, 5), Direction::Backward).move_home(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 4));
}
#[test]
fn move_home_errors_if_line_start_same_as_text_start_and_cursor_at_text_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    //assert!(Selection::new(0, 0).move_home(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(Range::new(0, 0), Direction::Forward).move_home(&text, CursorSemantics::Bar).is_err());
    //assert!(Selection::new(0, 1).move_home(&text, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(0, 1), Direction::Forward).move_home(&text, CursorSemantics::Block).is_err());
}
