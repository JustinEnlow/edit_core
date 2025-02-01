use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction};
use crate::selections::Selections;

#[test]
fn clear_non_primary_selections(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // normal use
    //let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 1, &text);
    //assert_eq!(Selections::new(vec![Selection::new(4, 4)], 0, &text), selections.clear_non_primary_selections().unwrap());
    assert_eq!(Selections::new(vec![Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text), selections.clear_non_primary_selections().unwrap());
}
#[test]
fn clear_non_primary_selections_errors_if_only_one_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    //assert!(Selections::new(vec![Selection::new(0, 0)], 0, &text).clear_non_primary_selections().is_err());
    assert!(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text).clear_non_primary_selections().is_err());
}
