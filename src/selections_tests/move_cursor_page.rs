use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};
use crate::selections::Selections;
use crate::view::View;

//         move page up
//         move page down
//         extend page up
//         extend page down

//             1          2           3           4
// 012 34567 89012 3456789012 34567 8901 2345 67890 12
// idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n 

// 0: idk
// 1: some
// 2: shit
// 3: something
// 4: else
// 5: and
// 6: yet
// 7: more
// 8: 

// block semantics
#[test] fn works_with_move_page_up_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n");
    let view = View::new(0, 0, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_page(&text, &view, CursorSemantics::Block, Selection::move_page_up).unwrap()
    );
}
#[test] fn works_with_move_page_down_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n");
    let view = View::new(0, 6, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(42, 43), Direction::Forward), Selection::new(Range::new(37, 38), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(42, 43), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_page(&text, &view, CursorSemantics::Block, Selection::move_page_down).unwrap()
    );
}
#[test] fn works_with_extend_page_up_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n");
    let view = View::new(0, 0, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 5), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_page(&text, &view, CursorSemantics::Block, Selection::extend_page_up).unwrap()
    );
}
#[test] fn works_with_extend_page_down_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n");
    let view = View::new(0, 6, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(37, 38), Direction::Forward), Selection::new(Range::new(41, 42), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(37, 42), Direction::Forward, 4)], 0, &text),
        selections.move_cursor_page(&text, &view, CursorSemantics::Block, Selection::extend_page_down).unwrap()
    );
}
#[test] fn should_error_if_single_selection_and_results_in_same_state_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n");
    
    let view = View::new(0, 0, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_page(&text, &view, CursorSemantics::Block, Selection::move_page_up).is_err());
    assert!(selections.move_cursor_page(&text, &view, CursorSemantics::Block, Selection::extend_page_up).is_err());

    let view = View::new(0, 6, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(42, 43), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_page(&text, &view, CursorSemantics::Block, Selection::move_page_down).is_err());

    let selections = Selections::new(vec![Selection::new(Range::new(41, 42), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_page(&text, &view, CursorSemantics::Block, Selection::extend_page_down).is_err());
}



// bar semantics
#[test] fn works_with_move_page_up_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n");
    let view = View::new(0, 0, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 0), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_page(&text, &view, CursorSemantics::Bar, Selection::move_page_up).unwrap()
    );
}
#[test] fn works_with_move_page_down_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n");
    let view = View::new(0, 6, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(42, 42), Direction::Forward), Selection::new(Range::new(37, 37), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(42, 42), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_page(&text, &view, CursorSemantics::Bar, Selection::move_page_down).unwrap()
    );
}
#[test] fn works_with_extend_page_up_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n");
    let view = View::new(0, 0, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(0, 4), Direction::Backward, 0)], 0, &text),
        selections.move_cursor_page(&text, &view, CursorSemantics::Bar, Selection::extend_page_up).unwrap()
    );
}
#[test] fn works_with_extend_page_down_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n");
    let view = View::new(0, 6, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(42, 42), Direction::Forward), Selection::new(Range::new(37, 37), Direction::Forward)], 0, &text);
    assert_eq!(
        Selections::new(vec![Selection::with_stored_line_position(Range::new(37, 42), Direction::Forward, 0)], 0, &text),
        selections.move_cursor_page(&text, &view, CursorSemantics::Bar, Selection::extend_page_down).unwrap()
    );
}
#[test] fn should_error_if_single_selection_and_results_in_same_state_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\nsomething\nelse\nand\nyet\nmore\n");
    
    let view = View::new(0, 0, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_page(&text, &view, CursorSemantics::Bar, Selection::move_page_up).is_err());
    assert!(selections.move_cursor_page(&text, &view, CursorSemantics::Bar, Selection::extend_page_up).is_err());

    let view = View::new(0, 6, 3, 3);
    let selections = Selections::new(vec![Selection::new(Range::new(42, 42), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_page(&text, &view, CursorSemantics::Bar, Selection::move_page_down).is_err());

    let selections = Selections::new(vec![Selection::new(Range::new(42, 42), Direction::Forward)], 0, &text);
    assert!(selections.move_cursor_page(&text, &view, CursorSemantics::Bar, Selection::extend_page_down).is_err());
}
