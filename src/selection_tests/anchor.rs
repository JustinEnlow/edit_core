use crate::{range::Range, selection::{Selection, Direction}};

#[test] fn when_anchor_same_as_head(){
    //assert_eq!(0, Selection::new(0, 0).anchor());
    assert_eq!(0, Selection::new(Range::new(0, 0), Direction::Forward).anchor());
}

#[test] fn when_head_greater_than_anchor(){
    //assert_eq!(0, Selection::new(0, 1).anchor());
    assert_eq!(0, Selection::new(Range::new(0, 1), Direction::Forward).anchor());
}

#[test] fn when_anchor_greater_than_head(){
    //assert_eq!(1, Selection::new(1, 0).anchor());
    assert_eq!(1, Selection::new(Range::new(0, 1), Direction::Backward).anchor());
}
