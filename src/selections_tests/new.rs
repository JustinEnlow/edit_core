use ropey::Rope;
use crate::selection::Selection;
use crate::selections::Selections;

#[test] fn selections_are_grapheme_aligned(){
    assert!(false);
}

#[test] fn selections_are_sorted_by_ascending_position_in_doc(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(2, 4), Selection::new(0, 5), Selection::new(3, 6)], 0, &text);
    let expected = Selections::new(vec![Selection::new(0, 5), Selection::new(2, 4), Selection::new(3, 6)], 1, &text);
    assert_eq!(expected, selections);
}

#[test] fn overlapping_selections_are_merged(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 5), Selection::new(2, 4), Selection::new(3, 6)], 1, &text);
    let expected = Selections::new(vec![Selection::with_stored_line_position(0, 6, 2)], 0, &text);
    assert_eq!(expected, selections);
}

#[test]#[should_panic] fn ensures_all_selections_are_within_doc_bounds(){
    let text = Rope::from("idk\nsome\nshit\n");
    Selections::new(vec![Selection::new(0, 0), Selection::new(19, 19)], 0, &text);  //panics
}

#[test]
#[should_panic]
fn should_panic_if_input_selections_empty(){
    let text = Rope::from("idk\nsome\nshit\n");
    Selections::new(vec![], 0, &text);  //panics
}
