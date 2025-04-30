use ropey::Rope;
use crate::range::Range;
use crate::selection::{Selection, CursorSemantics, Direction};

#[test] fn when_head_equal_anchor_bar_semantics(){
    let semantics = CursorSemantics::Bar;
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(0, Selection::new(Range::new(0, 0), Direction::Forward).cursor(&text, semantics));   //|>idk\nsome\nshit\n
    assert_eq!(0, Selection::new_from_components(0, 0, None, &text, semantics).cursor(&text, semantics));   //|>idk\nsome\nshit\n
}

// anchor != head not asserted in Selection::new() for block semantics
//#[test] fn when_head_equal_anchor_block_semantics(){
//    let semantics = CursorSemantics::Block;
//    let text = Rope::from("idk\nsome\nshit\n");
//    
//    assert_eq!(0, Selection::new(Range::new(0, 0), Direction::Forward).cursor(&text, semantics));   //though this state should be impossible with block cursor semantics
//    assert_eq!(0, Selection::new_from_components(0, 0, None, &text, semantics).cursor(&text, semantics));   //though this state should be impossible with block cursor semantics
//    
//    assert_eq!(1, Selection::new(Range::new(2, 2), Direction::Forward).cursor(&text, semantics));   //i:d|>k\nsome\nshit\n   //though this state should be impossible with block cursor semantics
//    assert_eq!(1, Selection::new_from_components(2, 2, None, &text, semantics).cursor(&text, semantics));   //i:d|>k\nsome\nshit\n   //though this state should be impossible with block cursor semantics
//}

#[test] fn when_head_greater_than_anchor_bar_semantics(){
    let semantics = CursorSemantics::Bar;
    let text = Rope::from("idk\nsome\nshit\n");

    assert_eq!(2, Selection::new(Range::new(1, 2), Direction::Forward).cursor(&text, semantics));   //i|d>k\nsome\nshit\n
    assert_eq!(2, Selection::new_from_components(1, 2, None, &text, semantics).cursor(&text, semantics));   //i|d>k\nsome\nshit\n
}
#[test] fn when_head_greater_than_anchor_block_semantics(){
    let semantics = CursorSemantics::Block;
    let text = Rope::from("idk\nsome\nshit\n");

    assert_eq!(1, Selection::new(Range::new(1, 2), Direction::Forward).cursor(&text, semantics));   //i|:d>k\nsome\nshit\n
    assert_eq!(1, Selection::new_from_components(1, 2, None, &text, semantics).cursor(&text, semantics));   //i|:d>k\nsome\nshit\n
}

#[test] fn when_anchor_greater_than_head_bar_semantics(){
    let semantics = CursorSemantics::Bar;
    let text = Rope::from("idk\nsome\nshit\n");

    assert_eq!(1, Selection::new(Range::new(1, 2), Direction::Backward).cursor(&text, semantics));  //i<d|k\nsome\nshit\n
    assert_eq!(1, Selection::new_from_components(2, 1, None, &text, semantics).cursor(&text, semantics));   //i<d|k\nsome\nshit\n
}
#[test] fn when_anchor_greater_than_head_block_semantics(){
    let semantics = CursorSemantics::Block;
    let text = Rope::from("idk\nsome\nshit\n");
    
    assert_eq!(1, Selection::new(Range::new(1, 2), Direction::Backward).cursor(&text, semantics));  //i:<d|k\nsome\nshit\n
    assert_eq!(1, Selection::new_from_components(2, 1, None, &text, semantics).cursor(&text, semantics));   //i:<d|k\nsome\nshit\n
}
