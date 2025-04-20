//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//
//#[test]
//fn move_right(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    
//    // normal use
//    //assert_eq!(Selection::new(0, 0).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(1, 1, 1));
//    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(1, 1), Direction::Forward, 1));
//    //assert_eq!(Selection::new(0, 1).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(1, 2, 1));
//    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(1, 2), Direction::Forward, 1));
//    //TODO: assert_eq!(Selection::new(1, 0).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 1, 1));
//    
//    // new line resets stored line position
//    //assert_eq!(Selection::new(3, 3).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
//    assert_eq!(Selection::new(Range::new(3, 3), Direction::Forward).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 0));
//    //assert_eq!(Selection::new(3, 4).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
//    assert_eq!(Selection::new(Range::new(3, 4), Direction::Forward).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0));
//    
//    // with selection extended, collapses selection, then performs move
//    //assert_eq!(Selection::new(0, 3).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
//    assert_eq!(Selection::new(Range::new(0, 3), Direction::Forward).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 0));
//    //assert_eq!(Selection::new(3, 0).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(1, 1, 1));
//    assert_eq!(Selection::new(Range::new(0, 3), Direction::Backward).move_right(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(1, 1), Direction::Forward, 1));
//    //assert_eq!(Selection::new(0, 3).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));
//    assert_eq!(Selection::new(Range::new(0, 3), Direction::Forward).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 3));
//    //assert_eq!(Selection::new(3, 0).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(1, 2, 1));
//    assert_eq!(Selection::new(Range::new(0, 3), Direction::Backward).move_right(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(1, 2), Direction::Forward, 1));
//}
//#[test]
//fn move_right_errors_if_at_doc_end(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    //assert!(Selection::new(14, 14).move_right(&text, CursorSemantics::Bar).is_err());
//    assert!(Selection::new(Range::new(14, 14), Direction::Forward).move_right(&text, CursorSemantics::Bar).is_err());
//    //assert!(Selection::new(14, 15).move_right(&text, CursorSemantics::Block).is_err());
//    assert!(Selection::new(Range::new(14, 15), Direction::Forward).move_right(&text, CursorSemantics::Block).is_err());
//}
//