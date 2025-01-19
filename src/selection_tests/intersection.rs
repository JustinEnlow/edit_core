use crate::selection::Selection;

#[test] fn intersection(){
    let first = Selection::new(0, 6);
    let second = Selection::new(3, 9);
    assert!(first.intersection(&second).is_ok());
    assert_eq!(Selection::new(3, 6), first.intersection(&second).unwrap());
    
    let first = Selection::new(1, 5);
    let second = Selection::new(2, 3);
    assert!(first.intersection(&second).is_ok());
    assert_eq!(Selection::new(2, 3), first.intersection(&second).unwrap());
}
#[test] fn intersection_should_error_if_non_overlapping(){
    let first = Selection::new(0, 4);
    let second = Selection::new(5, 9);
    assert!(first.intersection(&second).is_err());
}
