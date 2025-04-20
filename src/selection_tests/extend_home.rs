//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//
//#[test]
//fn extend_home(){
//    let text = Rope::from("    idk\n");
//    
//    // extends selection to text start when head past text start
//    //assert_eq!(Selection::with_stored_line_position(6, 4, 4), Selection::new(6, 6).extend_home(&text, CursorSemantics::Bar).unwrap());
//    assert_eq!(Selection::with_stored_line_position(Range::new(4, 6), Direction::Backward, 4), Selection::new(Range::new(6, 6), Direction::Forward).extend_home(&text, CursorSemantics::Bar).unwrap());
//    //assert_eq!(Selection::with_stored_line_position(7, 4, 4), Selection::new(6, 7).extend_home(&text, CursorSemantics::Block).unwrap());
//    assert_eq!(Selection::with_stored_line_position(Range::new(4, 7), Direction::Backward, 4), Selection::new(Range::new(6, 7), Direction::Forward).extend_home(&text, CursorSemantics::Block).unwrap());
//    
//    // extends selection to line start when head at text start
//    //assert_eq!(Selection::with_stored_line_position(4, 0, 0), Selection::new(4, 4).extend_home(&text, CursorSemantics::Bar).unwrap());
//    assert_eq!(Selection::with_stored_line_position(Range::new(0, 4), Direction::Backward, 0), Selection::new(Range::new(4, 4), Direction::Forward).extend_home(&text, CursorSemantics::Bar).unwrap());
//    //assert_eq!(Selection::with_stored_line_position(5, 0, 0), Selection::new(4, 5).extend_home(&text, CursorSemantics::Block).unwrap());   //    [:i]dk\n  //:]    i[dk\n
//    assert_eq!(Selection::with_stored_line_position(Range::new(0, 5), Direction::Backward, 0), Selection::new(Range::new(4, 5), Direction::Forward).extend_home(&text, CursorSemantics::Block).unwrap());   //    [:i]dk\n  //:]    i[dk\n
//    
//    // extends selection to text start when head before text start
//    //assert_eq!(Selection::with_stored_line_position(1, 4, 4), Selection::new(1, 1).extend_home(&text, CursorSemantics::Bar).unwrap());
//    assert_eq!(Selection::with_stored_line_position(Range::new(1, 4), Direction::Forward, 4), Selection::new(Range::new(1, 1), Direction::Forward).extend_home(&text, CursorSemantics::Bar).unwrap());
//    //assert_eq!(Selection::with_stored_line_position(1, 5, 4), Selection::new(1, 2).extend_home(&text, CursorSemantics::Block).unwrap()); // [: ]  idk\n  // [   :i]dk\n
//    assert_eq!(Selection::with_stored_line_position(Range::new(1, 5), Direction::Forward, 4), Selection::new(Range::new(1, 2), Direction::Forward).extend_home(&text, CursorSemantics::Block).unwrap()); // [: ]  idk\n  // [   :i]dk\n
//}
//#[test]
//fn extend_home_errors_if_line_start_same_as_text_start_and_cursor_at_text_start(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    //assert!(Selection::new(0, 0).extend_home(&text, CursorSemantics::Bar).is_err());
//    assert!(Selection::new(Range::new(0, 0), Direction::Forward).extend_home(&text, CursorSemantics::Bar).is_err());
//    //assert!(Selection::new(0, 1).extend_home(&text, CursorSemantics::Block).is_err());
//    assert!(Selection::new(Range::new(0, 1), Direction::Forward).extend_home(&text, CursorSemantics::Block).is_err());
//}
//