use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};
use crate::selections::Selections;

/*
which movement fns can potentially overlap? test all...
    move up
    move down
    move left
    move right
    move cursor word boundary backward
    move cursor word boundary forward
    move line end
    move line start
    move line text start
    move home (switches between line start and line text start)
    extend up
    extend down
    extend left
    extend right
    extend word boundary backward
    extend word boundary forward
    extend line end
    extend line start
    extend line text start
    extend home (switches between line start and line text start)
    extend doc start
    extend doc end
    select line
*/

#[test] fn works_with_move_up_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text);
    assert_eq!(
        //Selections::new(vec![Selection::with_stored_line_position(0, 1, 0)], 0, &text),
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_up).unwrap()
    )
}
#[test] fn works_with_move_cursor_down_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(14, 15), Selection::new(13, 14)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(14, 15), Direction::Forward), Selection::new(Range::new(13, 14), Direction::Forward)], 0, &text);
    assert_eq!(
        //Selections::new(vec![Selection::with_stored_line_position(14, 15, 0)], 0, &text),
        Selections::new(vec![Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_down).unwrap()
    )
}
#[test] fn works_with_move_left_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(1, 2)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(1, 2), Direction::Forward)], 0, &text);
    assert_eq!(
        //Selections::new(vec![Selection::with_stored_line_position(0, 1, 0)], 0, &text),
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_left).unwrap()
    )
}
#[test] fn works_with_move_right_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(14, 15), Selection::new(13, 14)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(14, 15), Direction::Forward), Selection::new(Range::new(13, 14), Direction::Forward)], 0, &text);
    assert_eq!(
        //Selections::new(vec![Selection::with_stored_line_position(14, 15, 0)], 0, &text),
        Selections::new(vec![Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_right).unwrap()
    )
}
#[test] fn works_with_move_cursor_word_boundary_backward_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(1, 2)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(1, 2), Direction::Forward)], 0, &text);
    assert_eq!(
        //Selections::new(vec![Selection::with_stored_line_position(0, 1, 0)], 0, &text),
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_left_word_boundary).unwrap()
    )
}
#[test] fn works_with_move_cursor_word_boundary_forward_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(14, 15), Selection::new(13, 14)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(14, 15), Direction::Forward), Selection::new(Range::new(13, 14), Direction::Forward)], 0, &text);
    assert_eq!(
        //Selections::new(vec![Selection::with_stored_line_position(14, 15, 0)], 0, &text),
        Selections::new(vec![Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_right_word_boundary).unwrap()
    )
}
#[test] fn works_with_move_line_end_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(1, 2), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 3)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_line_text_end).unwrap()
    );
}
#[test] fn works_with_move_line_start_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Forward), Selection::new(Range::new(2, 3), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_line_start).unwrap()
    )
}
#[test] fn works_with_move_line_text_start_block_semantics(){
    let text = Rope::from("    idk\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(7, 8), Direction::Forward), Selection::new(Range::new(6, 7), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_line_text_start).unwrap()
    );
}
#[test] fn works_with_move_home_block_semantics(){
    let text = Rope::from("    idk\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(1, 2), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_home).unwrap()
    )
}

#[test] fn works_with_extend_up_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5)], 0, &text);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text);
    assert_eq!(
        //Selections::new(vec![Selection::with_stored_line_position(5, 1, 0)], 0, &text), //this is the correct check. make sure we have with_stored_line_position implemented properly
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 5), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_up).unwrap()
    )
}
#[test] fn works_with_extend_down_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(8, 9), Direction::Forward), Selection::new(Range::new(13, 14), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(8, 14), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_down).unwrap()
    );
}
#[test] fn works_with_extend_left_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(1, 2), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 2), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_left).unwrap()
    );
}
#[test] fn works_with_extend_right_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(13, 14), Direction::Forward), Selection::new(Range::new(12, 13), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(12, 14), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_right).unwrap()
    );
}
#[test] fn works_with_extend_word_boundary_backward_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(1, 2), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 2), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_left_word_boundary).unwrap()
    )
}
#[test] fn works_with_extend_word_boundary_forward_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(13, 14), Direction::Forward), Selection::new(Range::new(12, 13), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(12, 14), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_right_word_boundary).unwrap()
    )
}
#[test] fn works_with_extend_line_text_end_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(1, 2), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 2)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_line_text_end).unwrap()
    )
}
#[test] fn works_with_extend_line_start_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(2, 3), Direction::Forward), Selection::new(Range::new(3, 4), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_line_start).unwrap()
    )
}
#[test] fn works_with_extend_line_text_start_block_semantics(){
    let text = Rope::from("    idk\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(5, 6), Direction::Forward), Selection::new(Range::new(6, 7), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(4, 7), Direction::Backward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_line_text_start).unwrap()
    )
}
#[test] fn works_with_extend_home_block_semantics(){
    let text = Rope::from("    idk\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(1, 2), Direction::Forward), Selection::new(Range::new(2, 3), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(1, 5), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_home).unwrap()
    )
}
#[test] fn works_with_extend_doc_start_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(4, 5), Direction::Forward), Selection::new(Range::new(9, 10), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 10), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_doc_start).unwrap()
    );
}
#[test] fn works_with_extend_doc_end_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(4, 5), Direction::Forward), Selection::new(Range::new(9, 10), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(4, 14), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_doc_end).unwrap()
    );
}
#[test] fn works_with_select_line_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(1, 2), Direction::Forward), Selection::new(Range::new(2, 3), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 4), Direction::Forward, 3)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::select_line).unwrap()
    )
}

//should error if single selection and results in same state
#[test] fn should_error_if_single_selection_and_results_in_same_state_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");

    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_up).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_left).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_left_word_boundary).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_up).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_left).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_left_word_boundary).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_doc_start).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_line_start).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_line_text_start).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_home).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_line_start).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_line_text_start).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_home).is_err());

    let selections = Selections::new(vec![Selection::new(Range::new(14, 15), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_down).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_right).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_right_word_boundary).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::move_line_text_end).is_err());

    let selections = Selections::new(vec![Selection::new(Range::new(13, 14), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_down).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_right).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_right_word_boundary).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_doc_end).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::extend_line_text_end).is_err());

    let selections = Selections::new(vec![Selection::new(Range::new(0, 4), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Block, Selection::select_line).is_err());
}




#[test] fn works_with_move_up_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_up).unwrap()
    )
}
#[test] fn works_with_move_cursor_down_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(14, 14), Direction::Forward), Selection::new(Range::new(13, 13), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_down).unwrap()
    )
}
#[test] fn works_with_move_left_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_left).unwrap()
    )
}
#[test] fn works_with_move_right_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(14, 14), Direction::Forward), Selection::new(Range::new(13, 13), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_right).unwrap()
    )
}
#[test] fn works_with_move_cursor_word_boundary_backward_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_left_word_boundary).unwrap()
    )
}
#[test] fn works_with_move_cursor_word_boundary_forward_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(14, 14), Direction::Forward), Selection::new(Range::new(13, 13), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_right_word_boundary).unwrap()
    )
}
#[test] fn works_with_move_line_end_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(3, 3), Direction::Forward, 3)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_line_text_end).unwrap()
    );
}
#[test] fn works_with_move_line_start_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(3, 3), Direction::Forward), Selection::new(Range::new(2, 2), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_line_start).unwrap()
    )
}
#[test] fn works_with_move_line_text_start_bar_semantics(){
    let text = Rope::from("    idk\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(7, 7), Direction::Forward), Selection::new(Range::new(6, 6), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_line_text_start).unwrap()
    );
}
#[test] fn works_with_move_home_bar_semantics(){
    let text = Rope::from("    idk\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_home).unwrap()
    )
}

#[test] fn works_with_extend_up_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 4), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_up).unwrap()
    )
}
#[test] fn works_with_extend_down_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(9, 9), Direction::Forward), Selection::new(Range::new(14, 14), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(9, 14), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_down).unwrap()
    );
}
#[test] fn works_with_extend_left_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_left).unwrap()
    );
}
#[test] fn works_with_extend_right_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(14, 14), Direction::Forward), Selection::new(Range::new(13, 13), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(13, 14), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_right).unwrap()
    );
}
#[test] fn works_with_extend_word_boundary_backward_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_left_word_boundary).unwrap()
    )
}
#[test] fn works_with_extend_word_boundary_forward_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(14, 14), Direction::Forward), Selection::new(Range::new(13, 13), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(13, 14), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_right_word_boundary).unwrap()
    )
}
#[test] fn works_with_extend_line_text_end_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 3)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_line_text_end).unwrap()
    )
}
#[test] fn works_with_extend_line_start_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(2, 2), Direction::Forward), Selection::new(Range::new(3, 3), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 3), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_line_start).unwrap()
    )
}
#[test] fn works_with_extend_line_text_start_bar_semantics(){
    let text = Rope::from("    idk\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(5, 5), Direction::Forward), Selection::new(Range::new(6, 6), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(4, 6), Direction::Backward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_line_text_start).unwrap()
    )
}
#[test] fn works_with_extend_home_bar_semantics(){
    let text = Rope::from("    idk\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(1, 1), Direction::Forward), Selection::new(Range::new(2, 2), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(1, 4), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_home).unwrap()
    )
}
#[test] fn works_with_extend_doc_start_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(4, 4), Direction::Forward), Selection::new(Range::new(9, 9), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 9), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_doc_start).unwrap()
    );
}
#[test] fn works_with_extend_doc_end_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(4, 4), Direction::Forward), Selection::new(Range::new(9, 9), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(4, 14), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_doc_end).unwrap()
    );
}
#[test] fn works_with_select_line_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(Range::new(1, 1), Direction::Forward), Selection::new(Range::new(2, 2), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 4), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::select_line).unwrap()
    )
}

//should error if single selection and results in same state
#[test] fn should_error_if_single_selection_and_results_in_same_state_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");

    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_up).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_left).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_left_word_boundary).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_up).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_left).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_left_word_boundary).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_doc_start).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_line_start).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_line_text_start).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_home).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_line_start).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_line_text_start).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_home).is_err());

    let selections = Selections::new(vec![Selection::new(Range::new(14, 14), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_down).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_right).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_right_word_boundary).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_down).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_right).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_right_word_boundary).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_doc_end).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::move_line_text_end).is_err());
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::extend_line_text_end).is_err());

    let selections = Selections::new(vec![Selection::new(Range::new(0, 4), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_potentially_overlapping(&text, CursorSemantics::Bar, Selection::select_line).is_err());
}
