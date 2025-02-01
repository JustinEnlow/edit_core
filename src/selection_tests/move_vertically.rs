use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction, Movement};

#[test]
fn move_vertically(){
    let text = Rope::from("idk\nsomething\nelse\n");
    //assert_eq!(Selection::new(0, 0).move_vertically(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_vertically(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 0));
    //assert_eq!(Selection::new(4, 4).move_vertically(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(Range::new(4, 4), Direction::Forward).move_vertically(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 0).move_vertically(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 4, 0));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_vertically(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 4), Direction::Forward, 0));
    //assert_eq!(Selection::new(4, 4).move_vertically(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 0, 0));
    assert_eq!(Selection::new(Range::new(4, 4), Direction::Forward).move_vertically(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 4), Direction::Backward, 0));
    
    //assert_eq!(Selection::new(0, 1).move_vertically(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).move_vertically(1, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0));
    //assert_eq!(Selection::new(4, 5).move_vertically(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(Range::new(4, 5), Direction::Forward).move_vertically(1, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 1).move_vertically(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 5, 0));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).move_vertically(1, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 5), Direction::Forward, 0));
    //assert_eq!(Selection::new(4, 5).move_vertically(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(5, 0, 0));
    assert_eq!(Selection::new(Range::new(4, 5), Direction::Forward).move_vertically(1, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 5), Direction::Backward, 0));
    
    // handles moving/extending to text bounds correctly
    //assert_eq!(Selection::new(0, 0).move_vertically(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 19, 0)); //idk\nsomething\nelse\n[]
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_vertically(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(19, 19), Direction::Forward, 0)); //idk\nsomething\nelse\n[]
    //assert_eq!(Selection::new(19, 19).move_vertically(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));    //[]idk\nsomething\nelse\n
    assert_eq!(Selection::new(Range::new(19, 19), Direction::Forward).move_vertically(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0));    //[]idk\nsomething\nelse\n
    //assert_eq!(Selection::new(0, 0).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 19, 0));    //idk\nsomething\nelse\n[]
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 19), Direction::Forward, 0));    //idk\nsomething\nelse\n[]
    //assert_eq!(Selection::new(19, 19).move_vertically(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(19, 0, 0)); //[]idk\nsomething\nelse\n
    assert_eq!(Selection::new(Range::new(19, 19), Direction::Forward).move_vertically(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(0, 19), Direction::Backward, 0)); //[]idk\nsomething\nelse\n
    
    //assert_eq!(Selection::new(0, 1).move_vertically(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 20, 0));   //idk\nsomething\nelse\n|: >    //is this the desired functionality?...
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).move_vertically(19, &text, Movement::Move, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(19, 20), Direction::Forward, 0));   //idk\nsomething\nelse\n|: >    //is this the desired functionality?...
    //assert_eq!(Selection::new(19, 20).move_vertically(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
    assert_eq!(Selection::new(Range::new(19, 20), Direction::Forward).move_vertically(19, &text, Movement::Move, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 1).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 20, 0));
    //assert_eq!(Selection::new(19, 20).move_vertically(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(19, 0, 0));
    assert_eq!(Selection::new(Range::new(19, 20), Direction::Forward).move_vertically(19, &text, Movement::Extend, Direction::Backward, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(0, 19), Direction::Backward, 0));
}

#[test] fn errors_if_extended_outside_text_bounds(){
    let text = Rope::from("idk\nsomething\nelse\n");
    //assert!(Selection::new(0, 1).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(0, 1), Direction::Forward).move_vertically(19, &text, Movement::Extend, Direction::Forward, CursorSemantics::Block).is_err());
}
