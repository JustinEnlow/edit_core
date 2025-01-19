use crate::selection::Selection;

#[test] fn non_zero_width_selections(){
    // non zero width selections, no overlap
    assert_eq!(Selection::new(0, 3).overlaps(&Selection::new(3, 6)), false); //[idk]<\nso>me\nshit\n
    assert_eq!(Selection::new(0, 3).overlaps(&Selection::new(6, 3)), false); //[idk]>\nso<me\nshit\n
    assert_eq!(Selection::new(3, 0).overlaps(&Selection::new(3, 6)), false); //]idk[<\nso>me\nshit\n
    assert_eq!(Selection::new(3, 0).overlaps(&Selection::new(6, 3)), false); //]idk[>\nso<me\nshit\n
    assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(0, 3)), false); //<idk>[\nso]me\nshit\n
    assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(3, 0)), false); //>idk<[\nso]me\nshit\n
    assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(0, 3)), false); //<idk>]\nso[me\nshit\n
    assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(3, 0)), false); //>idk<]\nso[me\nshit\n
    
    // non-zero-width selections, overlap.
    assert_eq!(Selection::new(0, 4).overlaps(&Selection::new(3, 6)), true);  //[idk<\n]so>me\nshit\n
    assert_eq!(Selection::new(0, 4).overlaps(&Selection::new(6, 3)), true);  //[idk>\n]so<me\nshit\n
    assert_eq!(Selection::new(4, 0).overlaps(&Selection::new(3, 6)), true);  //]idk<\n[so>me\nshit\n
    assert_eq!(Selection::new(4, 0).overlaps(&Selection::new(6, 3)), true);  //]idk>\n[so<me\nshit\n
    assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(0, 4)), true);  //<idk[\n>so]me\nshit\n
    assert_eq!(Selection::new(3, 6).overlaps(&Selection::new(4, 0)), true);  //>idk[\n<so]me\nshit\n
    assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(0, 4)), true);  //<idk]\n>so[me\nshit\n
    assert_eq!(Selection::new(6, 3).overlaps(&Selection::new(4, 0)), true);  //>idk]\n<so[me\nshit\n
}

#[test] fn zero_width_and_non_zero_width_selection(){
    // Zero-width and non-zero-width selections, overlap.
    assert_eq!(Selection::new(0, 3).overlaps(&Selection::new(3, 3)), true);  //[idk<>]\nsome\nshit\n
    assert_eq!(Selection::new(3, 0).overlaps(&Selection::new(3, 3)), true);  //]idk<>[\nsome\nshit\n
    assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(0, 3)), true);  //<idk[]>\nsome\nshit\n
    assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(3, 0)), true);  //>idk[]<\nsome\nshit\n
    
    // Zero-width and non-zero-width selections, overlap.
    assert_eq!(Selection::new(1, 4).overlaps(&Selection::new(1, 1)), true);  //i[<>dk\n]some\nshit\n
    assert_eq!(Selection::new(4, 1).overlaps(&Selection::new(1, 1)), true);  //i]<>dk\n[some\nshit\n
    assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(1, 4)), true);  //i[<]dk\n>some\nshit\n
    assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(4, 1)), true);  //i[>]dk\n<some\nshit\n
    assert_eq!(Selection::new(1, 4).overlaps(&Selection::new(3, 3)), true);  //i[dk<>\n]some\nshit\n
    assert_eq!(Selection::new(4, 1).overlaps(&Selection::new(3, 3)), true);  //i]dk<>\n[some\nshit\n
    assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(1, 4)), true);  //i<dk[]\n>some\nshit\n
    assert_eq!(Selection::new(3, 3).overlaps(&Selection::new(4, 1)), true);  //i>dk[]\n<some\nshit\n
}

#[test] fn zero_width_selection(){
    // zero-width selections, no overlap.
    assert_eq!(Selection::new(0, 0).overlaps(&Selection::new(1, 1)), false); //[]i<>dk\nsome\nshit\n
    assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(0, 0)), false); //<>i[]dk\nsome\nshit\n
    
    // zero-width selections, overlap.
    assert_eq!(Selection::new(1, 1).overlaps(&Selection::new(1, 1)), true);  //i[<>]dk\nsome\nshit\n
}
