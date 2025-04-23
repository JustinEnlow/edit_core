use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, Direction, CursorSemantics};
use crate::selections::Selections;

#[ignore] #[test] fn selections_are_grapheme_aligned(){unimplemented!()}

#[test] fn selections_are_sorted_by_ascending_position_in_doc(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(2, 4), Direction::Forward), Selection::new(Range::new(0, 5), Direction::Forward), Selection::new(Range::new(3, 6), Direction::Forward)], 0, &text, CursorSemantics::Block);
    let expected = Selections::new(vec![Selection::new(Range::new(0, 5), Direction::Forward), Selection::new(Range::new(2, 4), Direction::Forward), Selection::new(Range::new(3, 6), Direction::Forward)], 1, &text, CursorSemantics::Block);
    assert_eq!(expected, selections);
}

#[test] fn overlapping_selections_are_merged(){unimplemented!()}

#[test] #[should_panic] fn should_panic_if_input_selections_empty(){
    let text = Rope::from("idk\nsome\nshit\n");
    let _ = Selections::new(vec![], 0, &text, CursorSemantics::Block);  //panics
}
