use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};
use crate::view::View;

#[test]
fn move_page_down(){
    //use edit_core::view::View;

    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    //assert_eq!(Selection::new(0, 0).move_page_down(&text, &client_view, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).move_page_down(&text, &client_view, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 0));
    //assert_eq!(Selection::new(0, 1).move_page_down(&text, &client_view, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(4, 5, 0));
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).move_page_down(&text, &client_view, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0));
}
#[test]
fn move_page_down_errors_if_already_on_bottommost_line(){
    //use edit_core::view::View;

    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    //assert!(Selection::new(14, 14).move_page_down(&text, &client_view, CursorSemantics::Bar).is_err());
    assert!(Selection::new(Range::new(14, 14), Direction::Forward).move_page_down(&text, &client_view, CursorSemantics::Bar).is_err());
    //assert!(Selection::new(14, 15).move_page_down(&text, &client_view, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(14, 15), Direction::Forward).move_page_down(&text, &client_view, CursorSemantics::Block).is_err());
}
