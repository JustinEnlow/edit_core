use ropey::Rope;
use crate::selection::Selection;
use crate::selections::Selections;

#[test] fn if_primary_is_other_than_last_new_primary_is_next(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5), Selection::new(9, 10)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(4, 5), Selection::new(9, 10)], 0, &text), selections.remove_primary_selection().unwrap());
}

#[test] fn if_primary_is_last_new_primary_is_one_less(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5), Selection::new(9, 10)], 2, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5)], 1, &text), selections.remove_primary_selection().unwrap());
}

#[test] fn errors_if_single_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 0)], 0, &text);
    assert!(selections.remove_primary_selection().is_err());
}
