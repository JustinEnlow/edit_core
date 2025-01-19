//#[test]
//fn set_direction_bar_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    assert_eq!(Selection::new(0, 0).set_direction(Direction::Forward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
//    assert_eq!(Selection::new(0, 0).set_direction(Direction::Backward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 0, 0));
//    assert_eq!(Selection::new(0, 5).set_direction(Direction::Backward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(5, 0, 0));
//    assert_eq!(Selection::new(5, 0).set_direction(Direction::Forward, &text, CursorSemantics::Bar), Selection::with_stored_line_position(0, 5, 1));
//}
//
//#[test]
//fn set_direction_block_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    assert_eq!(Selection::new(0, 1).set_direction(Direction::Backward, &text, CursorSemantics::Block), Selection::with_stored_line_position(1, 0, 0));
//    assert_eq!(Selection::new(1, 0).set_direction(Direction::Forward, &text, CursorSemantics::Block), Selection::with_stored_line_position(0, 1, 0));
//    assert_eq!(Selection::new(0, 5).set_direction(Direction::Backward, &text, CursorSemantics::Block), Selection::with_stored_line_position(5, 0, 0));
//    assert_eq!(Selection::new(5, 0).set_direction(Direction::Forward, &text, CursorSemantics::Block), Selection::with_stored_line_position(0, 5, 0));
//}
