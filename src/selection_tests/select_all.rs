//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//
//#[test]
//fn select_all(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    
//    //assert_eq!(Selection::with_stored_line_position(0, 14, 0), Selection::new(0, 0).select_all(&text, CursorSemantics::Bar).unwrap());
//    assert_eq!(Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 0), Selection::new(Range::new(0, 0), Direction::Forward).select_all(&text, CursorSemantics::Bar).unwrap());
//    //assert_eq!(Selection::with_stored_line_position(0, 14, 4), Selection::new(0, 1).select_all(&text, CursorSemantics::Block).unwrap());
//    assert_eq!(Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 4), Selection::new(Range::new(0, 1), Direction::Forward).select_all(&text, CursorSemantics::Block).unwrap());
//}
//#[test]
//fn select_all_errors_if_already_all_selected(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    //assert!(Selection::new(0, 14).select_all(&text, CursorSemantics::Bar).is_err());
//    assert!(Selection::new(Range::new(0, 14), Direction::Forward).select_all(&text, CursorSemantics::Bar).is_err());
//    //assert!(Selection::new(0, 14).select_all(&text, CursorSemantics::Block).is_err());
//    assert!(Selection::new(Range::new(0, 14), Direction::Forward).select_all(&text, CursorSemantics::Block).is_err());
//    //assert!(Selection::new(0, 15).select_all(&text, CursorSemantics::Block).is_err());  //though this shouldn't be a possible state //TODO: maybe add an assert in Selection::new() that guarantees this...
//    //assert!(Selection::new(Range::new(0, 15), Direction::Forward).select_all(&text, CursorSemantics::Block).is_err());  //though this shouldn't be a possible state //ensured in assert_invariants
//}
//