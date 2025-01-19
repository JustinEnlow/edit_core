use ropey::Rope;
use crate::selection::{Selection, CursorSemantics, Movement, Direction};

#[test]
fn move_horizontally(){
    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
    assert_eq!(Selection::new(0, 0).move_horizontally(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(1, 1, 1));
    assert_eq!(Selection::new(1, 1).move_horizontally(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(0, 0).move_horizontally(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 1, 1));
    assert_eq!(Selection::new(1, 1).move_horizontally(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(1, 0, 0));
    
    assert_eq!(Selection::new(0, 1).move_horizontally(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(1, 2, 1));
    assert_eq!(Selection::new(1, 2).move_horizontally(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(0, 1).move_horizontally(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 2, 1));
    assert_eq!(Selection::new(1, 2).move_horizontally(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 0, 0));
    
    // handles moving/extending to text bounds correctly
    assert_eq!(Selection::new(0, 0).move_horizontally(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 19, 0));
    assert_eq!(Selection::new(19, 19).move_horizontally(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(0, 0).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 19, 0));
    assert_eq!(Selection::new(19, 19).move_horizontally(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 0, 0));
    
    assert_eq!(Selection::new(0, 1).move_horizontally(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 20, 0));
    assert_eq!(Selection::new(19, 20).move_horizontally(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(0, 1).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 20, 0));
    assert_eq!(Selection::new(19, 20).move_horizontally(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 0, 0)); //:<idk\nsomething\nelse\n|
}
