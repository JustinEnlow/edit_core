use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};
use crate::selections::Selections;

/*
which movement fns can potentially overlap? test all...
    move up
    move down
    move left
    move right
    move cursor word boundary forward
    move cursor word boundary backward
    extend up
    extend down
    extend left
    extend right
    extend word boundary forward
    extend word boundary backward
*/

#[test] fn works_with_move_up_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(0, 1, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_up).unwrap()
    )
}
#[test] fn works_with_move_cursor_down_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(14, 15), Selection::new(13, 14)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(14, 15, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_down).unwrap()
    )
}
#[test] fn works_with_move_left_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(1, 2)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(0, 1, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_left).unwrap()
    )
}
#[test] fn works_with_move_right_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(14, 15), Selection::new(13, 14)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(14, 15, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_right).unwrap()
    )
}
#[test] fn works_with_move_cursor_word_boundary_forward(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(14, 15), Selection::new(13, 14)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(14, 15, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_right_word_boundary).unwrap()
    )
}
#[test] fn works_with_move_cursor_word_boundary_backward(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(1, 2)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(0, 1, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_left_word_boundary).unwrap()
    )
}

#[test] fn works_with_extend_up_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(5, 1, 0)], 0, &text), //this is the correct check. make sure we have with_stored_line_position implemented properly
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_up).unwrap()
    )
}
#[test] fn works_with_extend_down_block_semantics(){
    unimplemented!()
}
