use crate::range::Range;

#[test] #[should_panic] fn when_end_greater_than_start(){
    let _ = Range::new(1, 0);
}