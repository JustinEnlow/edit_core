use ropey::Rope;
use crate::selection::{Selection, CursorSemantics, SelectionError};

#[test]
fn extend_line_text_end(){
    let text = Rope::from("idk\n");
    assert_eq!(Selection::with_stored_line_position(0, 3, 3), Selection::new(0, 0).extend_line_text_end(&text, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 3, 2), Selection::new(0, 1).extend_line_text_end(&text, CursorSemantics::Block).unwrap());
}
#[test]
fn extend_line_text_end_errors_if_already_at_text_end(){
    let text = Rope::from("idk\n");
    assert!(Selection::new(3, 3).extend_line_text_end(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(2, 3).extend_line_text_end(&text, CursorSemantics::Block).is_err());
    assert_eq!(Selection::new(3, 4).extend_line_text_end(&text, CursorSemantics::Block), Err(SelectionError::ResultsInSameState));

    // repeating above test with subsequent text because a faulty implementation previously caused problems in this scenario. just making sure this doesn't happen again...
    let text = Rope::from("idk\nsomething\n");
    assert_eq!(Selection::new(3, 4).extend_line_text_end(&text, CursorSemantics::Block), Err(SelectionError::ResultsInSameState));
}
