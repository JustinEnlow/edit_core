use crate::selection::Selection;

#[test] fn contains(){
    assert!( Selection::new(0, 4).contains(3));
    assert!( Selection::new(4, 0).contains(3));
    assert!(!Selection::new(0, 4).contains(5));
    assert!(!Selection::new(4, 0).contains(5));
}
