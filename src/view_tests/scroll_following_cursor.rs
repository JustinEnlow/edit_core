use ropey::Rope;
use crate::view::View;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test]
fn scroll_following_cursor(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 2, 2);
    
    // return self when primary [`Selection`] `head` within [`View`] bounds
    //let selection = Selection::new(0, 0);
    let selection = Selection::new(Range::new(0, 0), Direction::Forward);
    assert_eq!(view, view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar));
    assert_eq!(String::from("id\nso\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar).text(&text));
    //let selection = Selection::new(0, 1);
    let selection = Selection::new(Range::new(0, 1), Direction::Forward);
    assert_eq!(view, view.scroll_following_cursor(&selection, &text, CursorSemantics::Block));
    assert_eq!(String::from("id\nso\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Block).text(&text));
    
    // returns proper [`View`] when [`Selection`] `head` outside [`View`] bounds
    //let selection = Selection::new(13, 13);
    let selection = Selection::new(Range::new(13, 13), Direction::Forward);
    assert_eq!(View::new(3, 1, 2, 2), view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar));
    assert_eq!(String::from("e\nt\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar).text(&text));
    //let selection = Selection::new(13, 14);
    let selection = Selection::new(Range::new(13, 14), Direction::Forward);
    assert_eq!(View::new(3, 1, 2, 2), view.scroll_following_cursor(&selection, &text, CursorSemantics::Block));
    assert_eq!(String::from("e\nt\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Block).text(&text));
}
