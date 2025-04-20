//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//
//#[test]
//fn extend_doc_start(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    //assert_eq!(Selection::with_stored_line_position(9, 0, 0), Selection::new(9, 9).extend_doc_start(&text, CursorSemantics::Bar).unwrap());
//    assert_eq!(Selection::with_stored_line_position(Range::new(0, 9), Direction::Backward, 0), Selection::new(Range::new(9, 9), Direction::Forward).extend_doc_start(&text, CursorSemantics::Bar).unwrap());
//    //assert_eq!(Selection::with_stored_line_position(10, 0, 0), Selection::new(9, 10).extend_doc_start(&text, CursorSemantics::Block).unwrap());  //idk\nsome\n[s]hit\n   //:]idk\nsome\ns[hit\n
//    assert_eq!(Selection::with_stored_line_position(Range::new(0, 10), Direction::Backward, 0), Selection::new(Range::new(9, 10), Direction::Forward).extend_doc_start(&text, CursorSemantics::Block).unwrap());  //idk\nsome\n[s]hit\n   //:]idk\nsome\ns[hit\n
//}
//#[test]
//fn extend_doc_start_errors_if_already_at_doc_start(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    //assert!(Selection::new(0, 0).extend_doc_start(&text, CursorSemantics::Bar).is_err());
//    assert!(Selection::new(Range::new(0, 0), Direction::Forward).extend_doc_start(&text, CursorSemantics::Bar).is_err());
//    //assert!(Selection::new(0, 1).extend_doc_start(&text, CursorSemantics::Block).is_err());
//    assert!(Selection::new(Range::new(0, 1), Direction::Forward).extend_doc_start(&text, CursorSemantics::Block).is_err());
//}
//