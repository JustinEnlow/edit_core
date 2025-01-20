use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};
use crate::selections::Selections;

#[test]
fn merge_overlapping_selections(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    let selections = Selections::new(vec![
        Selection::new(0, 2),    //[i d]k \n s o m e \n s h i t \n
        Selection::new(1, 4),    // i[d k \n]s o m e \n s h i t \n
        Selection::new(5, 7),    // i d k \n s[o m]e \n s h i t \n
        Selection::new(8, 10),   // i d k \n s o m e[\n s]h i t \n
        Selection::new(9, 12)    // i d k \n s o m e \n[s h i]t \n
    ], 4, &text);
    let expected_selections = Selections::new(vec![
        Selection::with_stored_line_position(0, 4, 0),    //[i d k \n]s o m e \n s h i t \n
        Selection::new(5, 7),    // i d k \n s[o m]e \n s h i t \n
        Selection::with_stored_line_position(8, 12, 3)    // i d k \n s o m e[\n s h i]t \n
    ], 2, &text);
    assert_eq!(expected_selections, selections.merge_overlapping(&text, CursorSemantics::Bar).unwrap());

    assert_eq!(Selections::new(vec![Selection::with_stored_line_position(0, 1, 0)], 0, &text), Selections::new(vec![Selection::new(0, 1), Selection::new(0, 1)], 0, &text).merge_overlapping(&text, CursorSemantics::Block).unwrap());
    assert_eq!(Selections::new(vec![Selection::with_stored_line_position(14, 0, 0)], 0, &text), Selections::new(vec![Selection::new(1, 0), Selection::new(14, 0)], 0, &text).merge_overlapping(&text, CursorSemantics::Block).unwrap());
    assert_eq!(Selections::new(vec![Selection::with_stored_line_position(0, 14, 4)], 0, &text), Selections::new(vec![Selection::new(0, 1), Selection::new(0, 14)], 0, &text).merge_overlapping(&text, CursorSemantics::Block).unwrap());
}

#[test] fn error(){
    assert!(false);
}
