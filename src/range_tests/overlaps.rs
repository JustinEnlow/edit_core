use crate::range::Range;

#[test] fn non_zero_width_ranges(){
    // non zero width ranges, no overlap
    assert_eq!(Range::new(0, 3).overlaps(&Range::new(3, 6)), false); //[idk]<\nso>me\nshit\n
    assert_eq!(Range::new(0, 3).overlaps(&Range::new(6, 3)), false); //[idk]>\nso<me\nshit\n
    assert_eq!(Range::new(3, 0).overlaps(&Range::new(3, 6)), false); //]idk[<\nso>me\nshit\n
    assert_eq!(Range::new(3, 0).overlaps(&Range::new(6, 3)), false); //]idk[>\nso<me\nshit\n
    assert_eq!(Range::new(3, 6).overlaps(&Range::new(0, 3)), false); //<idk>[\nso]me\nshit\n
    assert_eq!(Range::new(3, 6).overlaps(&Range::new(3, 0)), false); //>idk<[\nso]me\nshit\n
    assert_eq!(Range::new(6, 3).overlaps(&Range::new(0, 3)), false); //<idk>]\nso[me\nshit\n
    assert_eq!(Range::new(6, 3).overlaps(&Range::new(3, 0)), false); //>idk<]\nso[me\nshit\n
    
    // non-zero-width ranges, overlap.
    assert_eq!(Range::new(0, 4).overlaps(&Range::new(3, 6)), true);  //[idk<\n]so>me\nshit\n
    assert_eq!(Range::new(0, 4).overlaps(&Range::new(6, 3)), true);  //[idk>\n]so<me\nshit\n
    assert_eq!(Range::new(4, 0).overlaps(&Range::new(3, 6)), true);  //]idk<\n[so>me\nshit\n
    assert_eq!(Range::new(4, 0).overlaps(&Range::new(6, 3)), true);  //]idk>\n[so<me\nshit\n
    assert_eq!(Range::new(3, 6).overlaps(&Range::new(0, 4)), true);  //<idk[\n>so]me\nshit\n
    assert_eq!(Range::new(3, 6).overlaps(&Range::new(4, 0)), true);  //>idk[\n<so]me\nshit\n
    assert_eq!(Range::new(6, 3).overlaps(&Range::new(0, 4)), true);  //<idk]\n>so[me\nshit\n
    assert_eq!(Range::new(6, 3).overlaps(&Range::new(4, 0)), true);  //>idk]\n<so[me\nshit\n
}

#[test] fn zero_width_and_non_zero_width_range(){
    // Zero-width and non-zero-width ranges, overlap.
    assert_eq!(Range::new(0, 3).overlaps(&Range::new(3, 3)), true);  //[idk<>]\nsome\nshit\n
    assert_eq!(Range::new(3, 0).overlaps(&Range::new(3, 3)), true);  //]idk<>[\nsome\nshit\n
    assert_eq!(Range::new(3, 3).overlaps(&Range::new(0, 3)), true);  //<idk[]>\nsome\nshit\n
    assert_eq!(Range::new(3, 3).overlaps(&Range::new(3, 0)), true);  //>idk[]<\nsome\nshit\n
    
    // Zero-width and non-zero-width ranges, overlap.
    assert_eq!(Range::new(1, 4).overlaps(&Range::new(1, 1)), true);  //i[<>dk\n]some\nshit\n
    assert_eq!(Range::new(4, 1).overlaps(&Range::new(1, 1)), true);  //i]<>dk\n[some\nshit\n
    assert_eq!(Range::new(1, 1).overlaps(&Range::new(1, 4)), true);  //i[<]dk\n>some\nshit\n
    assert_eq!(Range::new(1, 1).overlaps(&Range::new(4, 1)), true);  //i[>]dk\n<some\nshit\n
    assert_eq!(Range::new(1, 4).overlaps(&Range::new(3, 3)), true);  //i[dk<>\n]some\nshit\n
    assert_eq!(Range::new(4, 1).overlaps(&Range::new(3, 3)), true);  //i]dk<>\n[some\nshit\n
    assert_eq!(Range::new(3, 3).overlaps(&Range::new(1, 4)), true);  //i<dk[]\n>some\nshit\n
    assert_eq!(Range::new(3, 3).overlaps(&Range::new(4, 1)), true);  //i>dk[]\n<some\nshit\n
}

#[test] fn zero_width_range(){
    // zero-width ranges, no overlap.
    assert_eq!(Range::new(0, 0).overlaps(&Range::new(1, 1)), false); //[]i<>dk\nsome\nshit\n
    assert_eq!(Range::new(1, 1).overlaps(&Range::new(0, 0)), false); //<>i[]dk\nsome\nshit\n
    
    // zero-width ranges, overlap.
    assert_eq!(Range::new(1, 1).overlaps(&Range::new(1, 1)), true);  //i[<>]dk\nsome\nshit\n
}
