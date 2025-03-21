use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction};
use crate::selections::Selections;

#[test]
fn decrement_primary_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // decrements
    //let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 1, &text);
    //assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text), selections.decrement_primary_selection().unwrap());
    assert_eq!(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 0, &text), selections.decrement_primary_selection().unwrap());
    
    // wraps on first selection
    //let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 0, &text);
    //assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 1, &text), selections.decrement_primary_selection().unwrap());
    assert_eq!(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 1, &text), selections.decrement_primary_selection().unwrap());
}
#[test]
fn decrement_primary_selection_errors_if_only_one_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    //assert!(Selections::new(vec![Selection::new(0, 0)], 0, &text).decrement_primary_selection().is_err());
    assert!(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text).decrement_primary_selection().is_err());
}
