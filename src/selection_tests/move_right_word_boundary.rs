//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//
//#[test]
//fn move_right_word_boundary(){
//    let text = Rope::from("use std::error::Error;");
//    //assert_eq!(Selection::with_stored_line_position(2, 3, 2), Selection::new(0, 1).move_right_word_boundary(&text, CursorSemantics::Block).unwrap());
//    assert_eq!(Selection::with_stored_line_position(Range::new(2, 3), Direction::Forward, 2), Selection::new(Range::new(0, 1), Direction::Forward).move_right_word_boundary(&text, CursorSemantics::Block).unwrap());
//    //assert_eq!(Selection::with_stored_line_position(3, 3, 3), Selection::new(0, 0).move_right_word_boundary(&text, CursorSemantics::Bar).unwrap());
//    assert_eq!(Selection::with_stored_line_position(Range::new(3, 3), Direction::Forward, 3), Selection::new(Range::new(0, 0), Direction::Forward).move_right_word_boundary(&text, CursorSemantics::Bar).unwrap());
//}
//#[test]
//fn move_right_word_boundary_errors_if_at_doc_end(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    //assert!(Selection::new(14, 15).move_right_word_boundary(&text, CursorSemantics::Block).is_err());
//    assert!(Selection::new(Range::new(14, 15), Direction::Forward).move_right_word_boundary(&text, CursorSemantics::Block).is_err());
//    //assert!(Selection::new(14, 14).move_right_word_boundary(&text, CursorSemantics::Bar).is_err());
//    assert!(Selection::new(Range::new(14, 14), Direction::Forward).move_right_word_boundary(&text, CursorSemantics::Bar).is_err());
//}
//