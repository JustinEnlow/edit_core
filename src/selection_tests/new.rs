use crate::range::Range;
use crate::selection::{Selection, Direction, CursorSemantics};
use ropey::Rope;

//TODO: maybe ensure all selection components are within text boundaries(take text: &Rope as input arg)
//TODO: maybe ensure semantics is set, and if block semantics, anchor != head(take semantics: CursorSemantics as input arg)

fn test_from_components(
    semantics: CursorSemantics, 
    text: &str, 
    selection: (usize, usize, Option<usize>), 
    expected_range_start: usize, 
    expected_range_end: usize,
    expected_anchor: usize,
    expected_head: usize,
    expected_direction: Direction
){
    let text = Rope::from(text);
    let anchor = selection.0;
    let head = selection.1;
    let stored_line_position = selection.2;
    let selection = Selection::new_from_components(anchor, head, stored_line_position, &text, semantics);
    assert_eq!(expected_range_start, selection.range.start);
    assert_eq!(expected_range_end, selection.range.end);
    assert_eq!(expected_anchor, selection.anchor());
    assert_eq!(expected_head, selection.head());
    assert_eq!(expected_direction, selection.direction);
}
//fn test_from_range(){}
//fn test_error_from_components(){}
//fn test_error_from_range(){}

#[test] fn no_extension_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(Range::new(0, 0), Direction::Forward);
    let selection = Selection::new_from_range(Range::new(0, 0), Direction::Forward, &text, CursorSemantics::Bar);
    assert_eq!(0, selection.range.start);
    assert_eq!(0, selection.range.end);
    assert_eq!(0, selection.anchor());
    assert_eq!(0, selection.head());
    assert_eq!(Direction::Forward, selection.direction);
    test_from_components(
        CursorSemantics::Bar, 
        "idk\nsome\nshit\n", 
        (0, 0, None), 
        0, 0, 
        0, 0, 
        Direction::Forward
    );
}
#[test] fn extended_forward_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(Range::new(0, 1), Direction::Forward);
    let selection = Selection::new_from_range(Range::new(0, 1), Direction::Forward, &text, CursorSemantics::Bar);
    assert_eq!(0, selection.range.start);
    assert_eq!(1, selection.range.end);
    assert_eq!(0, selection.anchor());
    assert_eq!(1, selection.head());
    assert_eq!(Direction::Forward, selection.direction);
}
#[test] fn extended_backward_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(Range::new(0, 1), Direction::Backward);
    let selection = Selection::new_from_range(Range::new(0, 1), Direction::Backward, &text, CursorSemantics::Bar);
    assert_eq!(0, selection.range.start);
    assert_eq!(1, selection.range.end);
    assert_eq!(1, selection.anchor());
    assert_eq!(0, selection.head());
    assert_eq!(Direction::Backward, selection.direction);
}

#[test] fn no_extension_forward_cursor_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(Range::new(0, 1), Direction::Forward);
    let selection = Selection::new_from_range(Range::new(0, 1), Direction::Forward, &text, CursorSemantics::Block);
    assert_eq!(0, selection.range.start);
    assert_eq!(1, selection.range.end);
    assert_eq!(0, selection.anchor());
    assert_eq!(1, selection.head());
    assert_eq!(Direction::Forward, selection.direction);
}
#[test] fn no_extension_backward_cursor_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(Range::new(0, 1), Direction::Backward);
    let selection = Selection::new_from_range(Range::new(0, 1), Direction::Backward, &text, CursorSemantics::Block);
    assert_eq!(0, selection.range.start);
    assert_eq!(1, selection.range.end);
    assert_eq!(1, selection.anchor());
    assert_eq!(0, selection.head());
    assert_eq!(Direction::Backward, selection.direction);
}
#[test] fn extended_forward_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(Range::new(0, 2), Direction::Forward);
    let selection = Selection::new_from_range(Range::new(0, 2), Direction::Forward, &text, CursorSemantics::Block);
    assert_eq!(0, selection.range.start);
    assert_eq!(2, selection.range.end);
    assert_eq!(0, selection.anchor());
    assert_eq!(2, selection.head());
    assert_eq!(Direction::Forward, selection.direction);
}
#[test] fn extended_backward_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(Range::new(0, 2), Direction::Backward);
    let selection = Selection::new_from_range(Range::new(0, 2), Direction::Backward, &text, CursorSemantics::Block);
    assert_eq!(0, selection.range.start);
    assert_eq!(2, selection.range.end);
    assert_eq!(2, selection.anchor());
    assert_eq!(0, selection.head());
    assert_eq!(Direction::Backward, selection.direction);
}
