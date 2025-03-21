use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction};

#[test] fn select_inside_instances_of_single_char(){
                         //0123456789012
    let text = Rope::from("  something  ");
    let selection = Selection::new(Range::new(5, 6), Direction::Forward);
    //println!("{:?}", text.slice(0..=selection.range.start).chars().reversed());
    assert_eq!(Selection::new(Range::new(2, 11), Direction::Forward), selection.select_inside_instances_of_single_char(' ', &text).unwrap());
}

#[test] fn select_inside_pair(){
                         //01234
    let text = Rope::from("(idk)");
    let selection = Selection::new(Range::new(2, 3), Direction::Forward);
    assert_eq!(Selection::new(Range::new(1, 4), Direction::Forward), selection.select_inside_pair('(', ')', &text).unwrap());
}
