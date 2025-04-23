use crate::range::Range;
use crate::selection::{Selection, Direction, CursorSemantics};

#[test] fn when_head_equals_anchor_bar_semantics(){
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).is_extended(CursorSemantics::Bar), false);
}
#[test] fn when_head_greater_than_anchor_bar_semantics(){
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).is_extended(CursorSemantics::Bar), true);
}
#[test] fn when_anchor_greater_than_head_bar_semantics(){
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Backward).is_extended(CursorSemantics::Bar), true);
}

#[test] fn when_head_equals_anchor_block_semantics(){
    assert_eq!(Selection::new(Range::new(0, 0), Direction::Forward).is_extended(CursorSemantics::Block), false);    //though this shouldn't be possible using block semantics
}
#[test] fn when_head_greater_than_anchor_block_semantics(){
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Forward).is_extended(CursorSemantics::Block), false);
    assert_eq!(Selection::new(Range::new(0, 2), Direction::Forward).is_extended(CursorSemantics::Block), true);
}
#[test] fn when_anchor_greater_than_head_block_semantics(){
    assert_eq!(Selection::new(Range::new(0, 1), Direction::Backward).is_extended(CursorSemantics::Block), false);
    assert_eq!(Selection::new(Range::new(0, 2), Direction::Backward).is_extended(CursorSemantics::Block), true);
    
}

//TODO: test with multichar graphemes
