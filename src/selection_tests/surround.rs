use crate::selection::{Selection, Direction};
use crate::range::Range;
use ropey::Rope;

#[test] fn with_non_extended_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(Range::new(4, 5), Direction::Forward);
    assert_eq!(vec![Selection::new(Range::new(4, 5), Direction::Forward), Selection::new(Range::new(5, 6), Direction::Forward)], selection.surround(&text));
}
#[test] fn with_extended_selection(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(Range::new(4, 8), Direction::Forward);
    assert_eq!(vec![Selection::new(Range::new(4, 5), Direction::Forward), Selection::new(Range::new(8, 9), Direction::Forward)], selection.surround(&text));
}
#[test] fn with_selection_at_text_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(Range::new(1, 4), Direction::Forward);
    assert_eq!(vec![Selection::new(Range::new(1, 2), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], selection.surround(&text));
}
#[test] fn with_selection_at_text_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(Range::new(13, 14), Direction::Forward);
    assert_eq!(vec![Selection::new(Range::new(13, 14), Direction::Forward), Selection::new(Range::new(14, 15), Direction::Forward)], selection.surround(&text));
}
#[test] fn with_selection_at_doc_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(Range::new(14, 15), Direction::Forward);
    assert!(selection.surround(&text).is_empty()); //should error?
}
