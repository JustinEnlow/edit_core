use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn move_doc_start(){
    let text = Rope::from("idk\n");
    assert_eq!(Selection::new(4, 4).move_doc_start(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));
    assert_eq!(Selection::new(4, 5).move_doc_start(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));
}
#[test]
fn move_doc_start_errors_if_already_at_doc_start(){
    let text = Rope::from("idk\n");
    assert!(Selection::new(0, 0).move_doc_start(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).move_doc_start(&text, CursorSemantics::Block).is_err());
}
