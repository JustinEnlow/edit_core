use ropey::Rope;
use crate::selection::Selection;
use crate::selections::Selections;

#[test]
fn push(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 0)], 0, &text); //[]idk\nsome\nshit\n
    assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text), selections.push(Selection::new(4, 4), true));
}
