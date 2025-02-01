use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction};
use crate::selections::Selections;

#[test] fn if_primary_is_other_than_last_new_primary_is_next(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5), Selection::new(9, 10)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward), Selection::new(Range::new(9, 10), Direction::Forward)], 0, &text);
    //assert_eq!(Selections::new(vec![Selection::new(4, 5), Selection::new(9, 10)], 0, &text), selections.remove_primary_selection().unwrap());
    assert_eq!(Selections::new(vec![Selection::new(Range::new(4, 5), Direction::Forward), Selection::new(Range::new(9, 10), Direction::Forward)], 0, &text), selections.remove_primary_selection().unwrap());
}

#[test] fn if_primary_is_last_new_primary_is_one_less(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5), Selection::new(9, 10)], 2, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward), Selection::new(Range::new(9, 10), Direction::Forward)], 2, &text);
    //assert_eq!(Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5)], 1, &text), selections.remove_primary_selection().unwrap());
    assert_eq!(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 1, &text), selections.remove_primary_selection().unwrap());
}

#[test] fn errors_if_single_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(0, 0)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text);
    assert!(selections.remove_primary_selection().is_err());
}
