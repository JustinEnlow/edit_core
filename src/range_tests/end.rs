use crate::range::Range;

#[test] fn idk(){
    assert_eq!(1, Range::new(0, 1).end);
}

#[test] #[should_panic] fn fails_when_end_less_than_start(){
    let _ = Range::new(1, 0);
}
