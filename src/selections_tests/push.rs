use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction};
use crate::selections::Selections;

#[test]
fn push(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(0, 0)], 0, &text); //[]idk\nsome\nshit\n
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text); //[]idk\nsome\nshit\n
    //assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text), selections.push(Selection::new(4, 4), true));
    assert_eq!(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 1, &text), selections.push(Selection::new(Range::new(4, 4), Direction::Forward), true));
}
