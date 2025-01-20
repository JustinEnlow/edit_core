use crate::range::Range;

#[test] fn intersection(){
    let first = Range::new(0, 6);
    let second = Range::new(3, 9);
    assert!(first.intersection(&second).is_some());
    assert_eq!(Range::new(3, 6), first.intersection(&second).unwrap());
    
    let first = Range::new(1, 5);
    let second = Range::new(2, 3);
    assert!(first.intersection(&second).is_some());
    assert_eq!(Range::new(2, 3), first.intersection(&second).unwrap());
}
#[test] fn intersection_should_error_if_non_overlapping(){
    let first = Range::new(0, 4);
    let second = Range::new(5, 9);
    assert!(first.intersection(&second).is_none());
}
