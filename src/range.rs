#[derive(Debug, PartialEq)]
pub enum RangeError{
    NoOverlap
}

/// A Range is a pair of indexes(indices?...) over any kind of underlying (linear? single dimensional) collection. (chars, graphemes, terminal cells, etc.)
#[derive(Debug, PartialEq, Clone)]
pub struct Range{
    pub start: usize,   // start should always be <= end
    pub end: usize      // end should always be >= start
}
impl Range{
    #[must_use] pub fn new(start: usize, end: usize) -> Self{   //or should this error if start > end instead?... maybe an assert makes sense here?
        //TODO: assert!(end >= start);      //this will help us weed out unnecessary tests. just comment them out as addressed here
        if start >= end{
            Self{start: end, end: start}
        }else{
            Self{start, end}
        }
    }
    
    /// Checks `self` and `other` for overlap.
    #[must_use] pub fn overlaps(&self, other: &Range) -> bool{
        self.start == other.start || 
        self.end == other.end || 
        (self.end > other.start && other.end > self.start)
    }
    
    /// Returns a bool indicating whether the provided index is contained within the [`Range`].
    #[must_use] pub fn contains(&self, idx: usize) -> bool{idx >= self.start && idx <= self.end}
    
    /// Returns a new [`Range`] representing the overlap of `self` and `other`. Returns `Option::None` if `self` and `other` are non-overlapping.
    #[must_use] pub fn intersection(&self, other: &Range) -> Option<Self>{
        if self.overlaps(other){
            Some(Range::new(self.start.max(other.start), self.end.min(other.end)))
        }else{None}
    }
    
    /// Create a new [`Range`] by merging self with other. Indiscriminate merge. merges whether overlapping, consecutive, contained, or disconnected entirely.
    #[must_use] pub fn merge(&self, other: &Range) -> Self{
        Range{start: self.start.min(other.start), end: self.end.max(other.end)}
    }
}
