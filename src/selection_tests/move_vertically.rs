use ropey::Rope;
use crate::selection::{Selection, CursorSemantics, Direction, Movement};

#[test]
fn move_vertically(){
    let text = Rope::from("idk\nsomething\nelse\n");
    assert_eq!(Selection::new(0, 0).move_vertically(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(4, 4).move_vertically(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(0, 0).move_vertically(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(4, 4).move_vertically(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 0, 0));
    
    assert_eq!(Selection::new(0, 1).move_vertically(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
    assert_eq!(Selection::new(4, 5).move_vertically(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(0, 1).move_vertically(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 5, 0));
    assert_eq!(Selection::new(4, 5).move_vertically(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(5, 0, 0));
    
    // handles moving/extending to text bounds correctly
    assert_eq!(Selection::new(0, 0).move_vertically(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 19, 0)); //idk\nsomething\nelse\n[]
    assert_eq!(Selection::new(19, 19).move_vertically(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));    //[]idk\nsomething\nelse\n
    assert_eq!(Selection::new(0, 0).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 19, 0));    //idk\nsomething\nelse\n[]
    assert_eq!(Selection::new(19, 19).move_vertically(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 0, 0)); //[]idk\nsomething\nelse\n
    
    assert_eq!(Selection::new(0, 1).move_vertically(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 20, 0));   //idk\nsomething\nelse\n|: >    //is this the desired functionality?...
    assert_eq!(Selection::new(19, 20).move_vertically(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(0, 1).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 20, 0));
    assert_eq!(Selection::new(19, 20).move_vertically(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 0, 0));
}
