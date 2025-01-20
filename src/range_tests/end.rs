use crate::range::Range;

#[test] fn idk(){
    assert_eq!(1, Range::new(0, 1).end);
    assert_eq!(1, Range::new(1, 0).end);
}
