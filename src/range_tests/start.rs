use crate::range::Range;

#[test] fn idk(){
    assert_eq!(0, Range::new(0, 1).start);
}

#[test] #[should_panic] fn fails_when_start_greater_than_end(){
    let _ = Range::new(1, 0);
}
