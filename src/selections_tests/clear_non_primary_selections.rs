use ropey::Rope;
use crate::selection::Selection;
use crate::selections::Selections;

#[test]
fn clear_non_primary_selections(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // normal use
    let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text);
    assert_eq!(Selections::new(vec![Selection::new(4, 4)], 0, &text), selections.clear_non_primary_selections().unwrap());
}
#[test]
fn clear_non_primary_selections_errors_if_only_one_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selections::new(vec![Selection::new(0, 0)], 0, &text).clear_non_primary_selections().is_err());
}
