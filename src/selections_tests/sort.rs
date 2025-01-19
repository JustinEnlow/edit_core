use ropey::Rope;
use crate::selection::Selection;
use crate::selections::Selections;

#[test]
fn sort(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![
        Selection::new(2, 4),
        Selection::new(0, 5),
        Selection::new(3, 6)
    ], 0, &text);
    let expected_selections = Selections::new(vec![
        Selection::new(0, 5),
        Selection::new(2, 4),
        Selection::new(3, 6)
    ], 1, &text);
    assert_eq!(expected_selections, selections.sort());
}
