use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

#[test] fn when_head_equal_anchor_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(0, 0).cursor(&text, CursorSemantics::Bar), 0);   //|>idk\nsome\nshit\n
}
#[test] fn when_head_equal_anchor_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(0, 0).cursor(&text, CursorSemantics::Block), 0);                          //though this state should be impossible with block cursor semantics
    assert_eq!(Selection::new(2, 2).cursor(&text, CursorSemantics::Block), 1); //i:d|>k\nsome\nshit\n   //though this state should be impossible with block cursor semantics
}

#[test] fn when_head_greater_than_anchor_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(1, 2).cursor(&text, CursorSemantics::Bar), 2); //i|d>k\nsome\nshit\n
}
#[test] fn when_head_greater_than_anchor_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(1, 2).cursor(&text, CursorSemantics::Block), 1); //i|:d>k\nsome\nshit\n
}

#[test] fn when_anchor_greater_than_head_bar_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(2, 1).cursor(&text, CursorSemantics::Bar), 1); //i<d|k\nsome\nshit\n
}
#[test] fn when_anchor_greater_than_head_block_semantics(){
    let text = Rope::from("idk\nsome\nshit\n");
    assert_eq!(Selection::new(2, 1).cursor(&text, CursorSemantics::Block), 1); //i:<d|k\nsome\nshit\n
}
