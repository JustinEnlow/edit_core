use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn extend_doc_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::with_stored_line_position(0, 14, 0), Selection::new(0, 0).extend_doc_end(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 15, 0), Selection::new(0, 1).extend_doc_end(&text, CursorSemantics::Block).unwrap());
}
#[test]
fn extend_doc_end_errors_if_already_at_doc_end(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(14, 14).extend_doc_end(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(14, 15).extend_doc_end(&text, CursorSemantics::Block).is_err());
}
