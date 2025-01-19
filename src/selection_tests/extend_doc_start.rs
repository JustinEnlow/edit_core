use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn extend_doc_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::with_stored_line_position(9, 0, 0), Selection::new(9, 9).extend_doc_start(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(10, 0, 0), Selection::new(9, 10).extend_doc_start(&text, CursorSemantics::Block).unwrap());  //idk\nsome\n[s]hit\n   //:]idk\nsome\ns[hit\n
}
#[test]
fn extend_doc_start_errors_if_already_at_doc_start(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 0).extend_doc_start(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).extend_doc_start(&text, CursorSemantics::Block).is_err());
}
