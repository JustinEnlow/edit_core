use ropey::Rope;
use crate::view::View;
use crate::Position;
use crate::selection::{Selection, CursorSemantics};
use crate::selections::Selections;

#[test]
fn primary_cursor_position(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 5, 5);
    let selections = Selections::new(vec![Selection::new(0, 3)], 0, &text);
    assert_eq!(Some(Position::new(3, 0)), view.primary_cursor_position(&text, &selections, CursorSemantics::Bar));

    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 5, 5);
    let selections = Selections::new(vec![Selection::new(0, 3)], 0, &text);
    assert_eq!(Some(Position::new(2, 0)), view.primary_cursor_position(&text, &selections, CursorSemantics::Block));
}
#[test]
fn primary_cursor_position_with_cursor_outside_view(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 5, 1);
    let selections = Selections::new(vec![Selection::new(9, 13)], 0, &text);
    assert_eq!(None, view.primary_cursor_position(&text, &selections, CursorSemantics::Bar));

    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 5, 1);
    let selections = Selections::new(vec![Selection::new(9, 13)], 0, &text);
    assert_eq!(None, view.primary_cursor_position(&text, &selections, CursorSemantics::Block));
}
