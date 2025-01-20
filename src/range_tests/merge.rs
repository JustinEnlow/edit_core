use crate::range::Range;

#[test]
fn merge(){
    // verify non extended Ranges
    assert_eq!(Range::new(0, 0).merge(&Range::new(0, 0)), Range::new(0, 0));
    assert_eq!(Range::new(0, 1).merge(&Range::new(0, 1)), Range::new(0, 1));
    assert_eq!(Range::new(1, 0).merge(&Range::new(1, 0)), Range::new(1, 0));

    // errors when direction of first Range is different than direction of other Range
    //assert!(Range::new(0, 1).merge(&Range::new(1, 0)));
    //assert!(Range::new(0, 1).merge(&Range::new(1, 0)));
    //assert!(Range::new(1, 0).merge(&Range::new(0, 1)));
    //assert!(Range::new(1, 0).merge(&Range::new(0, 1)));

    // when self.anchor > self.head && other.anchor > other.head                //this can't happen anymore with Range
    //assert_eq!(Range::new(4, 0).merge(&Range::new(5, 1)), Range::new(5, 0));
    //assert_eq!(Range::new(4, 0).merge(&Range::new(5, 1)), Range::new(5, 0));
    //assert_eq!(Range::new(5, 1).merge(&Range::new(4, 0)), Range::new(5, 0));
    //assert_eq!(Range::new(5, 1).merge(&Range::new(4, 0)), Range::new(5, 0));
    
    // when self.anchor < self.head && other.anchor < other.head
    assert_eq!(Range::new(0, 4).merge(&Range::new(1, 5)), Range::new(0, 5));
    assert_eq!(Range::new(0, 4).merge(&Range::new(1, 5)), Range::new(0, 5));
    assert_eq!(Range::new(1, 5).merge(&Range::new(0, 4)), Range::new(0, 5));
    assert_eq!(Range::new(1, 5).merge(&Range::new(0, 4)), Range::new(0, 5));
}
                    
#[test]
fn consecutive(){
    assert_eq!(Range::new(0, 1).merge(&Range::new(1, 2)), Range::new(0, 2));
    assert_eq!(Range::new(0, 1).merge(&Range::new(1, 2)), Range::new(0, 2));   //TODO: these using block semantics aren't really consecutive
    
    assert_eq!(Range::new(1, 0).merge(&Range::new(2, 1)), Range::new(2, 0));
    assert_eq!(Range::new(1, 0).merge(&Range::new(2, 1)), Range::new(2, 0));
    
    assert_eq!(Range::new(1, 2).merge(&Range::new(0, 1)), Range::new(0, 2));
    assert_eq!(Range::new(1, 2).merge(&Range::new(0, 1)), Range::new(0, 2));
    
    assert_eq!(Range::new(2, 1).merge(&Range::new(1, 0)), Range::new(2, 0));
    assert_eq!(Range::new(2, 1).merge(&Range::new(1, 0)), Range::new(2, 0));
}
                    
#[test]
fn overlapping(){
    assert_eq!(Range::new(0, 2).merge(&Range::new(1, 4)), Range::new(0, 4));
    assert_eq!(Range::new(0, 2).merge(&Range::new(1, 4)), Range::new(0, 4));
    
    assert_eq!(Range::new(2, 0).merge(&Range::new(4, 1)), Range::new(4, 0));
    assert_eq!(Range::new(2, 0).merge(&Range::new(4, 1)), Range::new(4, 0));
    
    assert_eq!(Range::new(1, 4).merge(&Range::new(0, 2)), Range::new(0, 4));
    assert_eq!(Range::new(1, 4).merge(&Range::new(0, 2)), Range::new(0, 4));
    
    assert_eq!(Range::new(4, 1).merge(&Range::new(2, 0)), Range::new(4, 0));
    assert_eq!(Range::new(4, 1).merge(&Range::new(2, 0)), Range::new(4, 0));
}
                    
#[test]
fn contained(){
    assert_eq!(Range::new(0, 6).merge(&Range::new(2, 4)), Range::new(0, 6));
    assert_eq!(Range::new(0, 6).merge(&Range::new(2, 4)), Range::new(0, 6));
    
    assert_eq!(Range::new(6, 0).merge(&Range::new(4, 2)), Range::new(6, 0));
    assert_eq!(Range::new(6, 0).merge(&Range::new(4, 2)), Range::new(6, 0));
    
    assert_eq!(Range::new(2, 4).merge(&Range::new(0, 6)), Range::new(0, 6));
    assert_eq!(Range::new(2, 4).merge(&Range::new(0, 6)), Range::new(0, 6));
    
    assert_eq!(Range::new(4, 2).merge(&Range::new(6, 0)), Range::new(6, 0));
    assert_eq!(Range::new(4, 2).merge(&Range::new(6, 0)), Range::new(6, 0));
}
                    
#[test]
fn disconnected(){
    assert_eq!(Range::new(0, 2).merge(&Range::new(4, 6)), Range::new(0, 6));
    assert_eq!(Range::new(0, 2).merge(&Range::new(4, 6)), Range::new(0, 6));
    
    assert_eq!(Range::new(2, 0).merge(&Range::new(6, 4)), Range::new(6, 0));
    assert_eq!(Range::new(2, 0).merge(&Range::new(6, 4)), Range::new(6, 0));
    
    assert_eq!(Range::new(4, 6).merge(&Range::new(0, 2)), Range::new(0, 6));
    assert_eq!(Range::new(4, 6).merge(&Range::new(0, 2)), Range::new(0, 6));
    
    assert_eq!(Range::new(6, 4).merge(&Range::new(2, 0)), Range::new(6, 0));
    assert_eq!(Range::new(6, 4).merge(&Range::new(2, 0)), Range::new(6, 0));
}
