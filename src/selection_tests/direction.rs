use crate::selection::{Selection, Direction};

#[test] fn when_head_equals_anchor_bar_semantics(){
    assert_eq!(Selection::new(0, 0).direction, Direction::Forward);
}
#[test] fn when_head_greater_than_anchor_bar_semantics(){
    assert_eq!(Selection::new(0, 1).direction, Direction::Forward);
}
#[test] fn when_anchor_greater_than_head_bar_semantics(){
    assert_eq!(Selection::new(1, 0).direction, Direction::Backward);
}

#[test] fn when_head_equals_anchor_block_semantics(){
    assert_eq!(Selection::new(0, 0).direction, Direction::Forward);  //state shouldn't be possible with block cursor semantics, so result may be strange.
    //assert_eq!(Selection::new(1, 1).direction(&text, CursorSemantics::Block), Direction::Backward); //state shouldn't be possible with block cursor semantics, but the result is still valid.
    assert_eq!(Selection::new(1, 1).direction, Direction::Forward); //state shouldn't be possible with block cursor semantics, but the result is still valid.
}
#[test] fn when_head_greater_than_anchor_block_semantics(){
    assert_eq!(Selection::new(0, 1).direction, Direction::Forward);
}
#[test] fn when_anchor_greater_than_head_block_semantics(){
    assert_eq!(Selection::new(1, 0).direction, Direction::Backward);
}
