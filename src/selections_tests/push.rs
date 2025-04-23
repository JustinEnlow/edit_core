use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction, CursorSemantics};
use crate::selections::Selections;

#[test] fn push_works(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text, CursorSemantics::Bar); //[]idk\nsome\nshit\n
    assert_eq!(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 1, &text, CursorSemantics::Bar), selections.push(Selection::new(Range::new(4, 4), Direction::Forward), true));
}
