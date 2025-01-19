use crate::selection::Selection;

#[test] fn when_anchor_same_as_head(){
    assert_eq!(0, Selection::new(0, 0).head());
}

#[test] fn when_head_greater_than_anchor(){
    assert_eq!(1, Selection::new(0, 1).head());
}

#[test] fn when_anchor_greater_than_head(){
    assert_eq!(0, Selection::new(1, 0).head());
}
