use crate::range::Range;
use crate::selection::{Selection, Direction};

//TODO: maybe ensure all selection components are within text boundaries(take text: &Rope as input arg)
//TODO: maybe ensure semantics is set, and if block semantics, anchor != head(take semantics: CursorSemantics as input arg)

#[test] fn no_extension_bar_semantics(){
    let selection = Selection::new(Range::new(0, 0), Direction::Forward);
    assert_eq!(0, selection.range.start);
    assert_eq!(0, selection.range.end);
    assert_eq!(0, selection.anchor());
    assert_eq!(0, selection.head());
    assert_eq!(Direction::Forward, selection.direction);
}
#[test] fn extended_forward_bar_semantics(){
    let selection = Selection::new(Range::new(0, 1), Direction::Forward);
    assert_eq!(0, selection.range.start);
    assert_eq!(1, selection.range.end);
    assert_eq!(0, selection.anchor());
    assert_eq!(1, selection.head());
    assert_eq!(Direction::Forward, selection.direction);
}
#[test] fn extended_backward_bar_semantics(){
    let selection = Selection::new(Range::new(0, 1), Direction::Backward);
    assert_eq!(0, selection.range.start);
    assert_eq!(1, selection.range.end);
    assert_eq!(1, selection.anchor());
    assert_eq!(0, selection.head());
    assert_eq!(Direction::Backward, selection.direction);
}

#[test] fn no_extension_forward_cursor_block_semantics(){
    let selection = Selection::new(Range::new(0, 1), Direction::Forward);
    assert_eq!(0, selection.range.start);
    assert_eq!(1, selection.range.end);
    assert_eq!(0, selection.anchor());
    assert_eq!(1, selection.head());
    assert_eq!(Direction::Forward, selection.direction);
}
#[test] fn no_extension_backward_cursor_block_semantics(){
    let selection = Selection::new(Range::new(0, 1), Direction::Backward);
    assert_eq!(0, selection.range.start);
    assert_eq!(1, selection.range.end);
    assert_eq!(1, selection.anchor());
    assert_eq!(0, selection.head());
    assert_eq!(Direction::Backward, selection.direction);
}
#[test] fn extended_forward_block_semantics(){
    let selection = Selection::new(Range::new(0, 2), Direction::Forward);
    assert_eq!(0, selection.range.start);
    assert_eq!(2, selection.range.end);
    assert_eq!(0, selection.anchor());
    assert_eq!(2, selection.head());
    assert_eq!(Direction::Forward, selection.direction);
}
#[test] fn extended_backward_block_semantics(){
    let selection = Selection::new(Range::new(0, 2), Direction::Backward);
    assert_eq!(0, selection.range.start);
    assert_eq!(2, selection.range.end);
    assert_eq!(2, selection.anchor());
    assert_eq!(0, selection.head());
    assert_eq!(Direction::Backward, selection.direction);
}
