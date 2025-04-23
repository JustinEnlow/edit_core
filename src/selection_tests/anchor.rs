use crate::{
    selection::{Selection, Direction}, 
    range::Range
};

#[test] fn bar_semantics(){
    assert_eq!(0, Selection::new(Range::new(0, 0), Direction::Forward).anchor());
}

#[test] fn direction_forward_block_semantics(){
    assert_eq!(0, Selection::new(Range::new(0, 1), Direction::Forward).anchor());
}
#[test] fn direction_backward_block_semantics(){
    assert_eq!(1, Selection::new(Range::new(0, 1), Direction::Backward).anchor());
}