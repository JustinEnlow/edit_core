use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction};
use crate::selections::Selections;

#[test] fn sort_works(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![
        Selection::new(Range::new(2, 4), Direction::Forward),
        Selection::new(Range::new(0, 5), Direction::Forward),
        Selection::new(Range::new(3, 6), Direction::Forward)
    ], 0, &text);
    let expected_selections = Selections::new(vec![
        Selection::new(Range::new(0, 5), Direction::Forward),
        Selection::new(Range::new(2, 4), Direction::Forward),
        Selection::new(Range::new(3, 6), Direction::Forward)
    ], 1, &text);
    assert_eq!(expected_selections, selections.sort());
}
