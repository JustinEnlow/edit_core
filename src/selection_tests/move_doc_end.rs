//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//
//#[test]
//fn move_doc_end(){
//    let text = Rope::from("idk\nsome\nshit");
//    //assert_eq!(Selection::new(0, 0).move_doc_end(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(13, 13, 4));
//    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_doc_end(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(13, 13), Direction::Forward, 4));
//    //assert_eq!(Selection::new(0, 1).move_doc_end(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(13, 14, 4));
//    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).move_doc_end(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(13, 14), Direction::Forward, 4));
//}
//#[test]
//fn move_doc_end_errors_if_already_at_doc_end(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    //assert!(Selection::new(14, 14).move_doc_end(&text, CursorSemantics::Bar).is_err());
//    assert!(Selection::new(Range::new(14, 14), Direction::Forward).move_doc_end(&text, CursorSemantics::Bar).is_err());
//    //assert!(Selection::new(14, 15).move_doc_end(&text, CursorSemantics::Block).is_err());
//    assert!(Selection::new(Range::new(14, 15), Direction::Forward).move_doc_end(&text, CursorSemantics::Block).is_err());
//}
//