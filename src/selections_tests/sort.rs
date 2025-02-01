use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction};
use crate::selections::Selections;

#[test]
fn sort(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![
        //Selection::new(2, 4),
        Selection::new(Range::new(2, 4), Direction::Forward),
        //Selection::new(0, 5),
        Selection::new(Range::new(0, 5), Direction::Forward),
        //Selection::new(3, 6)
        Selection::new(Range::new(3, 6), Direction::Forward)
    ], 0, &text);
    let expected_selections = Selections::new(vec![
        //Selection::new(0, 5),
        Selection::new(Range::new(0, 5), Direction::Forward),
        //Selection::new(2, 4),
        Selection::new(Range::new(2, 4), Direction::Forward),
        //Selection::new(3, 6)
        Selection::new(Range::new(3, 6), Direction::Forward)
    ], 1, &text);
    assert_eq!(expected_selections, selections.sort());
}
