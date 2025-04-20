//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//use crate::selections::Selections;
//
// collapse selection
//#[test] fn works_with_collapse_selection_block_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Forward), Selection::new(Range::new(4, 8), Direction::Forward)], 0, &text);
//    assert_eq!(
//        Selections::new(vec![Selection::with_stored_line_position(Range::new(2, 3), Direction::Forward, 2), Selection::with_stored_line_position(Range::new(7, 8), Direction::Forward, 3)], 0, &text),
//        selections.move_cursor_non_overlapping(&text, CursorSemantics::Block, Selection::collapse).unwrap()
//    );
//}
//
//#[test] fn errors_if_single_selection_and_results_in_same_state_block_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, &text);
//    assert!(selections.move_cursor_non_overlapping(&text, CursorSemantics::Block, Selection::collapse).is_err())
//}
//#[test] fn errors_if_all_selections_result_in_same_state_block_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(1, 2), Direction::Forward)], 0, &text);
//    assert!(selections.move_cursor_non_overlapping(&text, CursorSemantics::Block, Selection::collapse).is_err())
//}
//
//
//
//#[test] fn works_with_collapse_selection_bar_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Forward), Selection::new(Range::new(4, 8), Direction::Forward)], 0, &text);
//    assert_eq!(
//        Selections::new(vec![Selection::with_stored_line_position(Range::new(3, 3), Direction::Forward, 3), Selection::with_stored_line_position(Range::new(8, 8), Direction::Forward, 4)], 0, &text),
//        selections.move_cursor_non_overlapping(&text, CursorSemantics::Bar, Selection::collapse).unwrap()
//    );
//}
//
//#[test] fn errors_if_single_selection_and_results_in_same_state_bar_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text);
//    assert!(selections.move_cursor_non_overlapping(&text, CursorSemantics::Bar, Selection::collapse).is_err())
//}
//#[test] fn errors_if_all_selections_result_in_same_state_bar_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 1), Direction::Forward)], 0, &text);
//    assert!(selections.move_cursor_non_overlapping(&text, CursorSemantics::Bar, Selection::collapse).is_err())
//}
//