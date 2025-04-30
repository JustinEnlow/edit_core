use crate::{
    selection::{Selection, Direction, CursorSemantics}, 
    range::Range
};
use ropey::Rope;

#[test] fn bar_semantics(){
    assert_eq!(0, Selection::new(Range::new(0, 0), Direction::Forward).anchor());
    assert_eq!(0, Selection::new_from_components(0, 0, None, &Rope::from("idk\nsome\nshit\n"), CursorSemantics::Bar).anchor());
}

#[test] fn direction_forward_block_semantics(){
    assert_eq!(0, Selection::new(Range::new(0, 1), Direction::Forward).anchor());
    assert_eq!(0, Selection::new_from_components(0, 1, None, &Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block).anchor());
}
#[test] fn direction_backward_block_semantics(){
    assert_eq!(1, Selection::new(Range::new(0, 1), Direction::Backward).anchor());
    assert_eq!(1, Selection::new_from_components(1, 0, None, &Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block).anchor());
}