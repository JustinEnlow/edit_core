//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//use crate::selections::Selections;
//         move doc start   //though this could also use move_cursor_potentially_overlapping //may be more efficient to use clearing_non_primary
//         move doc end     //though this could also use move_cursor_potentially_overlapping //may be more efficient to use clearing_non_primary
//         select all       //though this could also use move_cursor_potentially_overlapping //may be more efficient to use clearing_non_primary


// block semantics
//#[test] fn works_with_move_doc_start_block_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(4, 5), Direction::Forward), Selection::new(Range::new(9, 10), Direction::Forward)], 0, &text);
//    assert_eq!(
//        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0)], 0, &text),
//        selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Block, Selection::move_doc_start).unwrap()
//    );
//}
//#[test] fn works_with_move_doc_end_block_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(4, 5), Direction::Forward), Selection::new(Range::new(9, 10), Direction::Forward)], 0, &text);
//    assert_eq!(
//        Selections::new(vec![Selection::with_stored_line_position(Range::new(14, 15), Direction::Forward, 0)], 0, &text),
//        selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Block, Selection::move_doc_end).unwrap()
//    );
//}
//#[test] fn works_with_select_all_block_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Forward), Selection::new(Range::new(9, 10), Direction::Forward)], 0, &text);
//    assert_eq!(
//        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 4)], 0, &text),
//        selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Block, Selection::select_all).unwrap()
//    )
//}
// errors
//#[test] fn errors_if_single_selection_results_in_same_state_block_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, &text);
//    assert!(selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Block, Selection::move_doc_start).is_err());
//
//    let selections = Selections::new(vec![Selection::new(Range::new(14, 15), Direction::Forward)], 0, &text);
//    assert!(selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Block, Selection::move_doc_end).is_err());
//
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 14), Direction::Forward)], 0, &text);
//    assert!(selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Block, Selection::select_all).is_err());
//}

// bar semantics
//#[test] fn works_with_move_doc_start_bar_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(4, 4), Direction::Forward), Selection::new(Range::new(9, 9), Direction::Forward)], 0, &text);
//    assert_eq!(
//        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0)], 0, &text),
//        selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Bar, Selection::move_doc_start).unwrap()
//    );
//}
//#[test] fn works_with_move_doc_end_bar_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(4, 4), Direction::Forward), Selection::new(Range::new(9, 9), Direction::Forward)], 0, &text);
//    assert_eq!(
//        Selections::new(vec![Selection::with_stored_line_position(Range::new(14, 14), Direction::Forward, 0)], 0, &text),
//        selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Bar, Selection::move_doc_end).unwrap()
//    );
//}
//#[test] fn works_with_select_all_bar_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    let selections = Selections::new(vec![Selection::new(Range::new(3, 3), Direction::Forward), Selection::new(Range::new(9, 9), Direction::Forward)], 0, &text);
//    assert_eq!(
//        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 14), Direction::Forward, 0)], 0, &text),
//        selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Bar, Selection::select_all).unwrap()
//    )
//}
// errors
//#[test] fn errors_if_single_selection_results_in_same_state_bar_semantics(){
//    let text = Rope::from("idk\nsome\nshit\n");
//
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text);
//    assert!(selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Bar, Selection::move_doc_start).is_err());
//
//    let selections = Selections::new(vec![Selection::new(Range::new(14, 14), Direction::Forward)], 0, &text);
//    assert!(selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Bar, Selection::move_doc_end).is_err());
//
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 14), Direction::Forward)], 0, &text);
//    assert!(selections.move_cursor_clearing_non_primary(&text, CursorSemantics::Bar, Selection::select_all).is_err());
//}
