use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Movement, Direction};

#[test]
fn put_cursor(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    //assert_eq!(Selection::new(0, 0).put_cursor(5, &text, Movement::Move, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(5, 5, 1));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).put_cursor(5, &text, Movement::Move, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(Range::new(5, 5), Direction::Forward, 1));
    //assert_eq!(Selection::new(5, 5).put_cursor(0, &text, Movement::Move, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(Range::new(5, 5), Direction::Forward).put_cursor(0, &text, Movement::Move, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0));
    
    //assert_eq!(Selection::new(0, 0).put_cursor(5, &text, Movement::Extend, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(0, 5, 1));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).put_cursor(5, &text, Movement::Extend, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 5), Direction::Forward, 1));
    //assert_eq!(Selection::new(5, 5).put_cursor(0, &text, Movement::Extend, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(5, 0, 0));
    assert_eq!(Selection::new(Range::new(5, 5), Direction::Forward).put_cursor(0, &text, Movement::Extend, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 5), Direction::Backward, 0));
    
    //assert_eq!(Selection::new(0, 1).put_cursor(5, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(5, 6, 1));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).put_cursor(5, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(Range::new(5, 6), Direction::Forward, 1));
    //assert_eq!(Selection::new(1, 0).put_cursor(5, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(5, 6, 1));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Backward).put_cursor(5, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(Range::new(5, 6), Direction::Forward, 1));
    //assert_eq!(Selection::new(5, 6).put_cursor(0, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(Range::new(5, 6), Direction::Forward).put_cursor(0, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0));
    //assert_eq!(Selection::new(6, 5).put_cursor(0, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(Range::new(5, 6), Direction::Backward).put_cursor(0, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0));
    
    //assert_eq!(Selection::new(0, 1).put_cursor(5, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 6, 1));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).put_cursor(5, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 6), Direction::Forward, 1));
    //assert_eq!(Selection::new(1, 0).put_cursor(5, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 6, 1));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Backward).put_cursor(5, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 6), Direction::Forward, 1));
    //assert_eq!(Selection::new(5, 6).put_cursor(0, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(6, 0, 0));
    assert_eq!(Selection::new(Range::new(5, 6), Direction::Forward).put_cursor(0, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 6), Direction::Backward, 0));
    //assert_eq!(Selection::new(6, 5).put_cursor(0, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(6, 0, 0));
    assert_eq!(Selection::new(Range::new(5, 6), Direction::Backward).put_cursor(0, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 6), Direction::Backward, 0));
    
    // test putting cursor at end of text
    //assert_eq!(Selection::new(0, 0).put_cursor(14, &text, Movement::Move, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(14, 14, 0));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).put_cursor(14, &text, Movement::Move, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 0).put_cursor(14, &text, Movement::Extend, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(0, 14, 0));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).put_cursor(14, &text, Movement::Extend, CursorSemantics::Bar, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 1).put_cursor(14, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(14, 15, 0));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).put_cursor(14, &text, Movement::Move, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 1).put_cursor(14, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 15, 0));  //can't extend a selection past text bounds
    //assert_eq!(Selection::new(0, 1).put_cursor(13, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(0, 14, 4));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).put_cursor(13, &text, Movement::Extend, CursorSemantics::Block, true).unwrap(), Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 4));
}

// now saturates at doc/text bounds
//#[test]
//fn put_cursor_errors_if_to_out_of_text_bounds(){
//    let text = Rope::from("idk\nsome\nshit\n"); //len 14
//    //assert!(Selection::new(0, 0).put_cursor(15, &text, Movement::Move, CursorSemantics::Bar, true).is_err());
//    assert!(Selection::new(Range::new(0, 0), Direction::Forward).put_cursor(15, &text, Movement::Move, CursorSemantics::Bar, true).is_err());
//    //assert!(Selection::new(0, 1).put_cursor(14, &text, Movement::Extend, CursorSemantics::Block, true).is_err());
//    assert!(Selection::new(Range::new(0, 1), Direction::Forward).put_cursor(14, &text, Movement::Extend, CursorSemantics::Block, true).is_err());
//    //assert!(Selection::new(0, 1).put_cursor(15, &text, Movement::Move, CursorSemantics::Block, true).is_err());
//    assert!(Selection::new(Range::new(0, 1), Direction::Forward).put_cursor(15, &text, Movement::Move, CursorSemantics::Block, true).is_err());
//    // TODO: test extend as well
//}
