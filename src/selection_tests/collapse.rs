use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test]
fn collapse(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // head < anchor
    assert_eq!(Selection::new(4, 0).collapse(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(0, 0, 0));  //<idk\n|some\nshit\n   //<|idk\nsome\nshit\n
    assert_eq!(Selection::new(4, 0).collapse(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(0, 1, 0));    //:<idk\n|some\nshit\n  //|:i>dk\nsome\nshit\n
    
    // anchor < head
    assert_eq!(Selection::new(0, 4).collapse(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(4, 4, 0));  //|idk\n>some\nshit\n   //idk\n|>some\nshit\n
    assert_eq!(Selection::new(0, 4).collapse(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(3, 4, 3));    //|idk\n>some\nshit\n   //idk|:\n>some\nshit\n
    
    // test setting cursor to end of text
    assert_eq!(Selection::new(0, 14).collapse(&text, CursorSemantics::Bar).unwrap(), Selection::with_stored_line_position(14, 14, 0));   //|idk\nsome\nshit\n>   //idk\nsome\nshit\n|>
    assert_eq!(Selection::new(0, 14).collapse(&text, CursorSemantics::Block).unwrap(), Selection::with_stored_line_position(13, 14, 4)); //|idk\nsome\nshit:\n>  //idk\nsome\nshit|:\n>
}
#[test]
fn collapse_errors_if_already_not_extended(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert!(Selection::new(0, 0).collapse(&text, CursorSemantics::Bar).is_err());
    assert!(Selection::new(0, 1).collapse(&text, CursorSemantics::Block).is_err());
}
