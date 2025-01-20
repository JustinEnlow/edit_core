use ropey::Rope;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test] fn when_head_equals_anchor_bar_semantics(){
    let text = Rope::from("idk");
    assert_eq!(Selection::new(0, 0).direction(&text, CursorSemantics::Bar), Direction::Forward);
}
#[test] fn when_head_greater_than_anchor_bar_semantics(){
    let text = Rope::from("idk");
    assert_eq!(Selection::new(0, 1).direction(&text, CursorSemantics::Bar), Direction::Forward);
}
#[test] fn when_anchor_greater_than_head_bar_semantics(){
    let text = Rope::from("idk");
    assert_eq!(Selection::new(1, 0).direction(&text, CursorSemantics::Bar), Direction::Backward);
}

#[test] fn when_head_equals_anchor_block_semantics(){
    let text = Rope::from("idk");
    assert_eq!(Selection::new(0, 0).direction(&text, CursorSemantics::Block), Direction::Forward);  //state shouldn't be possible with block cursor semantics, so result may be strange.
    //assert_eq!(Selection::new(1, 1).direction(&text, CursorSemantics::Block), Direction::Backward); //state shouldn't be possible with block cursor semantics, but the result is still valid.
    assert_eq!(Selection::new(1, 1).direction(&text, CursorSemantics::Block), Direction::Forward); //state shouldn't be possible with block cursor semantics, but the result is still valid.
}
#[test] fn when_head_greater_than_anchor_block_semantics(){
    let text = Rope::from("idk");
    assert_eq!(Selection::new(0, 1).direction(&text, CursorSemantics::Block), Direction::Forward);
}
#[test] fn when_anchor_greater_than_head_block_semantics(){
    let text = Rope::from("idk");
    assert_eq!(Selection::new(1, 0).direction(&text, CursorSemantics::Block), Direction::Backward);
}
