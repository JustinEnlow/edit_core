use crate::range::Range;

#[test] fn contains(){
    assert!( Range::new(0, 4).contains(3));
    assert!( Range::new(4, 0).contains(3));
    assert!(!Range::new(0, 4).contains(5));
    assert!(!Range::new(4, 0).contains(5));
}
