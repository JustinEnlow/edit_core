use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction, CursorSemantics};
use crate::selections::Selections;

#[test] fn pop_works(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(
        vec![
            Selection::new(Range::new(0, 0), Direction::Forward), 
            Selection::new(Range::new(1, 1), Direction::Forward)
        ], 0, &text, CursorSemantics::Bar
    );
    assert_eq!(
        Selections::new(
            vec![Selection::new(Range::new(0, 0), Direction::Forward)], 
            0, &text, CursorSemantics::Bar
        ), 
        selections.pop()
    );

    // always contains at least one selection
    let selections = Selections::new(
        vec![
            Selection::new(Range::new(0, 0), Direction::Forward)
        ], 0, &text, CursorSemantics::Bar
    );
    assert_eq!(
        Selections::new(
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward)
            ], 
            0, &text, CursorSemantics::Bar
        ), 
        selections.pop()
    );
}
