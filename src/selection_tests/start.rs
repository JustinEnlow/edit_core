use crate::selection::Selection;

#[test] fn when_head_greater_than_anchor(){
    assert_eq!(0, Selection::new(0, 4).start());
}

#[test] fn when_anchor_greater_than_head(){
    assert_eq!(0, Selection::new(4, 0).start());
}

#[test] fn when_anchor_same_as_head(){
    assert_eq!(0, Selection::new(0, 0).start());
}
