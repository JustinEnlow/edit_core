use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};
use crate::view::View;

#[test]
fn move_page_up(){
    //use edit_core::view::View;

    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    //assert_eq!(Selection::new(6, 6).move_page_up(&text, &client_view, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(2, 2, 2));
    assert_eq!(Selection::new(Range::new(6, 6), Direction::Forward).move_page_up(&text, &client_view, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(2, 2), Direction::Forward, 2));
    //assert_eq!(Selection::new(6, 7).move_page_up(&text, &client_view, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(2, 3, 2));
    assert_eq!(Selection::new(Range::new(6, 7), Direction::Forward).move_page_up(&text, &client_view, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(2, 3), Direction::Forward, 2));
}
#[test]
fn move_page_up_errors_if_already_on_topmost_line(){
    //use edit_core::view::View;

    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    //assert!(Selection::new(0, 0).move_page_up(&text, &client_view, CursorSemantics::Bar).is_err());
    assert!(Selection::new(Range::new(0, 0), Direction::Forward).move_page_up(&text, &client_view, CursorSemantics::Bar).is_err());
    //assert!(Selection::new(0, 1).move_page_up(&text, &client_view, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(0, 1), Direction::Forward).move_page_up(&text, &client_view, CursorSemantics::Block).is_err());
}
