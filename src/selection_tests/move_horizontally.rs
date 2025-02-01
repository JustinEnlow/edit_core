use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Movement, Direction};

#[test]
fn move_horizontally(){
    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
    //assert_eq!(Selection::new(0, 0).move_horizontally(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(1, 1, 1));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_horizontally(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(1, 1), Direction::Forward, 1));
    //assert_eq!(Selection::new(1, 1).move_horizontally(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(Range::new(1, 1), Direction::Forward).move_horizontally(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 0).move_horizontally(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 1, 1));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_horizontally(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 1));
    //assert_eq!(Selection::new(1, 1).move_horizontally(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(1, 0, 0));
    assert_eq!(Selection::new(Range::new(1, 1), Direction::Forward).move_horizontally(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Backward, 0));
    
    //assert_eq!(Selection::new(0, 1).move_horizontally(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(1, 2, 1));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).move_horizontally(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(1, 2), Direction::Forward, 1));
    //assert_eq!(Selection::new(1, 2).move_horizontally(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(Range::new(1, 2), Direction::Forward).move_horizontally(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 1).move_horizontally(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 2, 1));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).move_horizontally(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 2), Direction::Forward, 1));
    //assert_eq!(Selection::new(1, 2).move_horizontally(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 0, 0));
    assert_eq!(Selection::new(Range::new(1, 2), Direction::Forward).move_horizontally(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 2), Direction::Backward, 0));
    
    // handles moving/extending to text bounds correctly
    //assert_eq!(Selection::new(0, 0).move_horizontally(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 19, 0));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_horizontally(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(19, 19), Direction::Forward, 0));
    //assert_eq!(Selection::new(19, 19).move_horizontally(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(Range::new(19, 19), Direction::Forward).move_horizontally(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 0).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 19, 0));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 19), Direction::Forward, 0));
    //assert_eq!(Selection::new(19, 19).move_horizontally(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 0, 0));
    assert_eq!(Selection::new(Range::new(19, 19), Direction::Forward).move_horizontally(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 19), Direction::Backward, 0));
    
    //assert_eq!(Selection::new(0, 1).move_horizontally(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 20, 0));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).move_horizontally(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(19, 20), Direction::Forward, 0));
    //assert_eq!(Selection::new(19, 20).move_horizontally(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(Range::new(19, 20), Direction::Forward).move_horizontally(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 1).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 20, 0));
    //assert_eq!(Selection::new(19, 20).move_horizontally(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 0, 0)); //:<idk\nsomething\nelse\n|
    assert_eq!(Selection::new(Range::new(19, 20), Direction::Forward).move_horizontally(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 19), Direction::Backward, 0)); //:<idk\nsomething\nelse\n|
}

#[test] fn errors_if_extended_outside_text_bounds(){
    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
    //assert!(Selection::new(0, 1).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(0, 1), Direction::Forward).move_horizontally(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).is_err());
}
