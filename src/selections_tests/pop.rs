use ropey::Rope;
use crate::selection::Selection;
use crate::selections::Selections;

#[test]
fn works(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 0), Selection::new(1, 1)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0)], 0, &text), selections.pop());
    
    // always contains at least one selection
    let selections = Selections::new(vec![Selection::new(0, 0)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0)], 0, &text), selections.pop());
}
