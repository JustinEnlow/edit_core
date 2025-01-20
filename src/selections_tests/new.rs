use ropey::Rope;
use crate::selection::Selection;
use crate::selections::Selections;

// TODO: enable this test and make it work with new merge_overlapping impl
#[test]
fn grapheme_aligns_sorts_and_merges_valid_selections(){
    // sorts and merges overlapping
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![
        Selection::new(2, 4),    // i d[k \n]s o m e \n s h i t \n
        Selection::new(0, 5),    //[i d k \n s]o m e \n s h i t \n
        Selection::new(3, 6)     // i d k[\n s o]m e \n s h i t \n
    ], 0, &text);
    let expected_selections = Selections::new(vec![
        Selection::with_stored_line_position(0, 6, 2)     //[i d k \n s o]m e \n s h i t \n
    ], 0, &text);
    assert_eq!(expected_selections, selections);
}

#[test] fn selections_are_grapheme_aligned(){
    assert!(false);
}

#[test] fn selections_are_sorted_by_ascending_position_in_doc(){
    assert!(false);
}

#[test] fn overlapping_selections_are_merged(){
    assert!(false);
}

#[test] fn all_selections_are_within_doc_bounds(){
    assert!(false);
}

#[test]
#[should_panic]
fn should_panic_if_input_selections_empty(){
    let text = Rope::from("idk\nsome\nshit\n");
    Selections::new(vec![], 0, &text);  //panics
}
