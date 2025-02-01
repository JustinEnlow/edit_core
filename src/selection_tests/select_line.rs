use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test] fn normal_use_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(5, 6);
    let selection = Selection::new(Range::new(5, 6), Direction::Forward);
    //assert_eq!(Selection::new(4, 9), selection.select_line(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::new(Range::new(4, 9), Direction::Forward), selection.select_line(&text, CursorSemantics::Bar).unwrap());
}
#[test] fn normal_use_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(5, 6);
    let selection = Selection::new(Range::new(5, 6), Direction::Forward);
    //assert_eq!(Selection::new(4, 9), selection.select_line(&text, CursorSemantics::Block).unwrap());
    assert_eq!(Selection::new(Range::new(4, 9), Direction::Forward), selection.select_line(&text, CursorSemantics::Block).unwrap());
}

#[test] fn errors_if_selection_spans_multiple_lines_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(4, 12);
    let selection = Selection::new(Range::new(4, 12), Direction::Forward);
    assert!(selection.select_line(&text, CursorSemantics::Bar).is_err());
}
#[test] fn errors_if_selection_spans_multiple_lines_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(4, 12);
    let selection = Selection::new(Range::new(4, 12), Direction::Forward);
    assert!(selection.select_line(&text, CursorSemantics::Block).is_err());
}

#[test] fn errors_if_results_in_same_state_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(4, 9);
    let selection = Selection::new(Range::new(4, 9), Direction::Forward);
    assert!(selection.select_line(&text, CursorSemantics::Bar).is_err());
}
#[test] fn errors_if_results_in_same_state_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    //let selection = Selection::new(4, 9);
    let selection = Selection::new(Range::new(4, 9), Direction::Forward);
    assert!(selection.select_line(&text, CursorSemantics::Block).is_err());
}

#[test] fn errors_if_at_doc_end_and_line_empty(){
    let text = Rope::from("idk\nsome\nshit\n");
    //assert!(Selection::new(14, 14).select_line(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(Range::new(14, 14), Direction::Forward).select_line(&text, CursorSemantics::Bar).is_err());
    //assert!(Selection::new(14, 15).select_line(&text, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(14, 15), Direction::Forward).select_line(&text, CursorSemantics::Block).is_err());
}
