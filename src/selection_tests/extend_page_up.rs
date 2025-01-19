use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};
use crate::view::View;

#[test]
fn extend_page_up(){
    //use edit_core::view::View;
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert_eq!(Selection::with_stored_line_position(6, 2, 2), Selection::new(6, 6).extend_page_up(&text, &client_view, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(7, 2, 2), Selection::new(6, 7).extend_page_up(&text, &client_view, CursorSemantics::Block).unwrap());    //idk\nso[m]ething\nelse    //id:]k\nsom[ething\nelse
}
#[test]
fn extend_page_up_errors_if_on_topmost_line(){
    //use edit_core::view::View;
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert!(Selection::new(3, 3).extend_page_up(&text, &client_view, CursorSemantics::Bar).is_err());
    assert!(Selection::new(3, 4).extend_page_up(&text, &client_view, CursorSemantics::Block).is_err());
}
