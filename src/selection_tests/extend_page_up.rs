use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};
use crate::view::View;

#[test]
fn extend_page_up(){
    //use edit_core::view::View;
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    //assert_eq!(Selection::with_stored_line_position(6, 2, 2), Selection::new(6, 6).extend_page_up(&text, &client_view, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(Range::new(2, 6), Direction::Backward, 2), Selection::new(Range::new(6, 6), Direction::Forward).extend_page_up(&text, &client_view, CursorSemantics::Bar).unwrap());
    //assert_eq!(Selection::with_stored_line_position(7, 2, 2), Selection::new(6, 7).extend_page_up(&text, &client_view, CursorSemantics::Block).unwrap());    //idk\nso[m]ething\nelse    //id:]k\nsom[ething\nelse
    assert_eq!(Selection::with_stored_line_position(Range::new(2, 7), Direction::Backward, 2), Selection::new(Range::new(6, 7), Direction::Forward).extend_page_up(&text, &client_view, CursorSemantics::Block).unwrap());    //idk\nso[m]ething\nelse    //id:]k\nsom[ething\nelse
}
#[test]
fn extend_page_up_errors_if_on_topmost_line(){
    //use edit_core::view::View;
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    //assert!(Selection::new(3, 3).extend_page_up(&text, &client_view, CursorSemantics::Bar).is_err());
    assert!(Selection::new(Range::new(3, 3), Direction::Forward).extend_page_up(&text, &client_view, CursorSemantics::Bar).is_err());
    //assert!(Selection::new(3, 4).extend_page_up(&text, &client_view, CursorSemantics::Block).is_err());
    assert!(Selection::new(Range::new(3, 4), Direction::Forward).extend_page_up(&text, &client_view, CursorSemantics::Block).is_err());
}
