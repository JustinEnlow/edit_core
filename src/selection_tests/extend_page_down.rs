use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};
use crate::view::View;

#[test]
fn extend_page_down(){
    //use edit_core::view::View;
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert_eq!(Selection::with_stored_line_position(0, 4, 0), Selection::new(0, 0).extend_page_down(&text, &client_view, CursorSemantics::Bar).unwrap());
    assert_eq!(Selection::with_stored_line_position(0, 5, 0), Selection::new(0, 1).extend_page_down(&text, &client_view, CursorSemantics::Block).unwrap());  //[i]dk\nsomething\nelse    //[idk\n:s]omething\nelse
}
#[test]
fn extend_page_down_errors_if_on_bottommost_line(){
    //use edit_core::view::View;
    let text = Rope::from("idk\nsomething\nelse");
    let client_view = View::new(0, 0, 2, 2);
    assert!(Selection::new(14, 14).extend_page_down(&text, &client_view, CursorSemantics::Bar).is_err());
    assert!(Selection::new(14, 15).extend_page_down(&text, &client_view, CursorSemantics::Block).is_err());
}
