use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction};
use crate::selections::Selections;

#[test]
fn push_front(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(4, 4)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text);
    //assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 0, &text), selections.push_front(Selection::new(0, 0), true));
    assert_eq!(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text), selections.push_front(Selection::new(Range::new(0, 0), Direction::Forward), true));
    
    //let selections = Selections::new(vec![Selection::new(4, 4)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text);
    //assert_eq!(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text), selections.push_front(Selection::new(0, 0), false));
    assert_eq!(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 1, &text), selections.push_front(Selection::new(Range::new(0, 0), Direction::Forward), false));
}
