//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//
//#[test]
//fn move_line_text_start(){
//    let text = Rope::from("  idk\n");
//    //assert_eq!(Selection::new(0, 0).move_line_text_start(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(2, 2, 2));
//    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_line_text_start(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(2, 2), Direction::Forward, 2));
//    //assert_eq!(Selection::new(0, 1).move_line_text_start(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 3, 2));
//    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).move_line_text_start(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(2, 3), Direction::Forward, 2));
//}
//#[test]
//fn move_line_text_start_errors_if_already_at_line_text_start(){
//    let text = Rope::from("    idk\nsome\nshit\n");
//    //assert!(Selection::new(4, 4).move_line_text_start(&text, CursorSemantics::Bar).is_err());
//    assert!(Selection::new(Range::new(4, 4), Direction::Forward).move_line_text_start(&text, CursorSemantics::Bar).is_err());
//    //assert!(Selection::new(4, 5).move_line_text_start(&text, CursorSemantics::Block).is_err());
//    assert!(Selection::new(Range::new(4, 5), Direction::Forward).move_line_text_start(&text, CursorSemantics::Block).is_err());
//}
//