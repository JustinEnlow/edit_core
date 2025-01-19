use ropey::Rope;
use crate::selection::Selection;
use crate::selections::Selections;

#[test]
fn push_front(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(4, 4)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 0, &text), selections.push_front(Selection::new(0, 0), true));
    
    let selections = Selections::new(vec![Selection::new(4, 4)], 0, &text);
    assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text), selections.push_front(Selection::new(0, 0), false));
}
