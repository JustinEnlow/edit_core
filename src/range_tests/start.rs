use crate::range::Range;

#[test] fn idk(){
    assert_eq!(0, Range::new(0, 1).start);
    assert_eq!(0, Range::new(1, 0).start);
}
