use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test] fn normal_use_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(5, 6);
    assert_eq!(Selection::new(4, 9), selection.select_line(&text, CursorSemantics::Bar).unwrap());
}
#[test] fn normal_use_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(5, 6);
    assert_eq!(Selection::new(4, 9), selection.select_line(&text, CursorSemantics::Block).unwrap());
}

#[test] fn errors_if_selection_spans_multiple_lines_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(4, 12);
    assert!(selection.select_line(&text, CursorSemantics::Bar).is_err());
}
#[test] fn errors_if_selection_spans_multiple_lines_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(4, 12);
    assert!(selection.select_line(&text, CursorSemantics::Block).is_err());
}

#[test] fn errors_if_results_in_same_state_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(4, 9);
    assert!(selection.select_line(&text, CursorSemantics::Bar).is_err());
}
#[test] fn errors_if_results_in_same_state_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(4, 9);
    assert!(selection.select_line(&text, CursorSemantics::Block).is_err());
}

#[test] fn errors_if_at_doc_end_and_line_empty(){
    assert!(false);
}
