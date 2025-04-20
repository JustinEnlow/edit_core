//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//
//#[test]
//fn move_left(){
//    let text = Rope::from("idk\nsomething\nelse\n");
//    
//    // normal use
//    //assert_eq!(Selection::new(1, 1).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
//    assert_eq!(Selection::new(Range::new(1, 1), Direction::Forward).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0));
//    //assert_eq!(Selection::new(1, 2).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
//    assert_eq!(Selection::new(Range::new(1, 2), Direction::Forward).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0));
//    
//    // move to previous line resets stored line position
//    //assert_eq!(Selection::new(4, 4).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 3));
//    assert_eq!(Selection::new(Range::new(4, 4), Direction::Forward).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(3, 3), Direction::Forward, 3));
//    //assert_eq!(Selection::new(4, 5).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));
//    assert_eq!(Selection::new(Range::new(4, 5), Direction::Forward).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 3));
//    
//    // with selection extended, collapses selection, then performs move
//    //assert_eq!(Selection::new(1, 4).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 3, 3));
//    assert_eq!(Selection::new(Range::new(1, 4), Direction::Forward).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(3, 3), Direction::Forward, 3));
//    //assert_eq!(Selection::new(4, 1).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
//    assert_eq!(Selection::new(Range::new(1, 4), Direction::Backward).move_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0));
//    //assert_eq!(Selection::new(1, 4).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 3, 2));   // i[d k:\n]s o m e t h i n g \n e l s e
//    assert_eq!(Selection::new(Range::new(1, 4), Direction::Forward).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(2, 3), Direction::Forward, 2));   // i[d k:\n]s o m e t h i n g \n e l s e
//                                                                                                                                                                                                         // i d[k]\n s o m e t h i n g \n e l s e
//    //assert_eq!(Selection::new(4, 1).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));   // i]d k \n[s o m e t h i n g \n e l s e
//    assert_eq!(Selection::new(Range::new(1, 4), Direction::Backward).move_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0));   // i]d k \n[s o m e t h i n g \n e l s e
//                                                                                                                                                                                                          //[i]d k \n s o m e t h i n g \n e l s e
//}
//#[test]
//fn move_left_errors_if_at_doc_start(){
//    let text = Rope::from("idk\nsomething\nelse\n");
//    //assert!(Selection::new(0, 0).move_left(&text, CursorSemantics::Bar).is_err());
//    assert!(Selection::new(Range::new(0, 0), Direction::Forward).move_left(&text, CursorSemantics::Bar).is_err());
//    //assert!(Selection::new(0, 1).move_left(&text, CursorSemantics::Block).is_err());
//    assert!(Selection::new(Range::new(0, 1), Direction::Forward).move_left(&text, CursorSemantics::Block).is_err());
//}
//