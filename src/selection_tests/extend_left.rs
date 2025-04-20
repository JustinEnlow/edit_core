//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//
//#[test]
//fn extend_left(){
//    let text = Rope::from("idk\nsomething\nelse");
//    
//    // normal use
//    //assert_eq!(Selection::new(2, 2).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(2, 1, 1));
//    assert_eq!(Selection::new(Range::new(2, 2), Direction::Forward).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(1, 2), Direction::Backward, 1));
//    //assert_eq!(Selection::new(2, 3).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 1, 1)); //id[:k]\nsomething\nelse   //i:]dk[\nsomething\nelse
//    assert_eq!(Selection::new(Range::new(2, 3), Direction::Forward).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(1, 3), Direction::Backward, 1)); //id[:k]\nsomething\nelse   //i:]dk[\nsomething\nelse
//    //assert_eq!(Selection::new(3, 2).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 1, 1));    //so backward cursor behaves the same as forward
//    assert_eq!(Selection::new(Range::new(2, 3), Direction::Backward).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(1, 3), Direction::Backward, 1));    //so backward cursor behaves the same as forward
//    
//    //updates stored line position on line change
//    //assert_eq!(Selection::new(4, 4).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 3, 3));
//    assert_eq!(Selection::new(Range::new(4, 4), Direction::Forward).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(3, 4), Direction::Backward, 3));
//    //assert_eq!(Selection::new(4, 5).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(5, 3, 3)); //idk\n[s]omething\nelse    //idk:]\ns[omething\nelse
//    assert_eq!(Selection::new(Range::new(4, 5), Direction::Forward).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(3, 5), Direction::Backward, 3)); //idk\n[s]omething\nelse    //idk:]\ns[omething\nelse
//    
//    //previously extended
//    //assert_eq!(Selection::new(0, 3).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 2, 2));
//    assert_eq!(Selection::new(Range::new(0, 3), Direction::Forward).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 2), Direction::Forward, 2));
//    //assert_eq!(Selection::new(3, 1).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(3, 0, 0));
//    assert_eq!(Selection::new(Range::new(1, 3), Direction::Backward).extend_left(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0));
//    //assert_eq!(Selection::new(0, 3).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 2, 1)); //[id:k]\nsomething\nelse   //[i:d]k\nsomething\nelse
//    assert_eq!(Selection::new(Range::new(0, 3), Direction::Forward).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 2), Direction::Forward, 1)); //[id:k]\nsomething\nelse   //[i:d]k\nsomething\nelse
//    //assert_eq!(Selection::new(3, 1).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 0, 0)); //i:]dk[\nsomething\nelse   //:]idk[\nsomething\nelse
//    assert_eq!(Selection::new(Range::new(1, 3), Direction::Backward).extend_left(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0)); //i:]dk[\nsomething\nelse   //:]idk[\nsomething\nelse
//}
//#[test]
//fn extend_left_errors_if_at_doc_start(){
//    let text = Rope::from("idk\nsomething\nelse");
//    //assert!(Selection::new(0, 0).extend_left(&text, CursorSemantics::Bar).is_err());
//    assert!(Selection::new(Range::new(0, 0), Direction::Forward).extend_left(&text, CursorSemantics::Bar).is_err());
//    //assert!(Selection::new(0, 1).extend_left(&text, CursorSemantics::Block).is_err());
//    assert!(Selection::new(Range::new(0, 1), Direction::Forward).extend_left(&text, CursorSemantics::Block).is_err());
//}
//