//use crate::selection::{Selection, Direction};
//use crate::range::Range;
//use ropey::Rope;
//
//
////|i>d k ( s o m e [ ] _ t h i n g _ { } e l s e ) _ i d k     //no surrounding pair with cursor at this location
//#[test] fn at_start_with_no_surrounding_pair(){
//    let text = Rope::from("idk(some[] thing {}else) idk");
//    let selection = Selection::new(Range::new(0, 1), Direction::Forward);
//    assert!(selection.nearest_surrounding_pair(&text).is_empty());
//}
//// i d k ( s|o>m e [ ] _ t h i n g _ { } e l s e ) _ i d k     //paren surrounding pair with cursor at this location
//#[test] fn normal_case(){
//    let text = Rope::from("idk(some[] thing {}else) idk");
//    let selection = Selection::new(Range::new(5, 6), Direction::Forward);
//    assert_eq!(
//        vec![
//            Selection::new(Range::new(3, 4), Direction::Forward),
//            Selection::new(Range::new(23, 24), Direction::Forward)
//        ],
//        selection.nearest_surrounding_pair(&text)
//    );
//}
//// i d k ( s o m e|[>] _ t h i n g _ { } e l s e ) _ i d k     //square bracket surrounding pair with cursor at this location
//#[test] fn with_cursor_over_surrounding_pair_opening(){
//    let text = Rope::from("idk(some[] thing {}else) idk");
//    let selection = Selection::new(Range::new(8, 9), Direction::Forward);
//    assert_eq!(
//        vec![
//            Selection::new(Range::new(8, 9), Direction::Forward),
//            Selection::new(Range::new(9, 10), Direction::Forward)
//        ],
//        selection.nearest_surrounding_pair(&text)
//    );
//}
//// i d k ( s o m e [ ] _ t h|i>n g _ { } e l s e ) _ i d k     //paren surrounding pair with cursor at this location
//#[test] fn with_other_pairs_inside_surrounding_pair(){
//    let text = Rope::from("idk(some[] thing {}else) idk");
//    let selection = Selection::new(Range::new(13, 14), Direction::Forward);
//    assert_eq!(
//        vec![
//            Selection::new(Range::new(3, 4), Direction::Forward),
//            Selection::new(Range::new(23, 24), Direction::Forward)
//        ],
//        selection.nearest_surrounding_pair(&text)
//    );
//}
//// i d k ( s o m e [ ] _ t h i n g _ {|}>e l s e ) _ i d k     //curly bracket surrounding pair with cursor at this location
//#[test] fn with_cursor_over_surrounding_pair_closing(){
//    let text = Rope::from("idk(some[] thing {}else) idk");
//    let selection = Selection::new(Range::new(18, 19), Direction::Forward);
//    assert_eq!(
//        vec![
//            Selection::new(Range::new(17, 18), Direction::Forward),
//            Selection::new(Range::new(18, 19), Direction::Forward)
//        ],
//        selection.nearest_surrounding_pair(&text)
//    );
//}
//// i d k ( s o m e [ ] _ t h i n g _ { } e l s e ) _ i|d>k     //no surrounding pair with cursor at this location
//#[test] fn at_end_with_no_surrounding_pair(){
//    let text = Rope::from("idk(some[] thing {}else) idk");
//    let selection = Selection::new(Range::new(26, 27), Direction::Forward);
//    assert!(selection.nearest_surrounding_pair(&text).is_empty());
//}
//
//#[test] fn no_opening_bracket_pair_returns_empty_vec(){
//    let text = Rope::from("idk\nsomething)\n");
//    let selection = Selection::new(Range::new(3, 4), Direction::Forward);
//    assert!(
//        selection.nearest_surrounding_pair(&text).is_empty()
//    );
//}
//#[test] fn no_closing_bracket_pair_returns_empty_vec(){
//    let text = Rope::from("(idk\nsomething\n");
//    let selection = Selection::new(Range::new(3, 4), Direction::Forward);
//    assert!(
//        selection.nearest_surrounding_pair(&text).is_empty()
//    );
//}
//
////idk(some()t(h(i)n)g()else)    //test from multiple levels of same surrounding pair
//#[test] fn with_multiple_levels_of_same_surrounding_pair(){
//    let text = Rope::from("idk(some()t(h(i)n)g()else");
//    let selection = Selection::new(Range::new(12, 13), Direction::Forward);
//    assert_eq!(
//        vec![
//            Selection::new(Range::new(11, 12), Direction::Forward),
//            Selection::new(Range::new(17, 18), Direction::Forward)
//        ],
//        selection.nearest_surrounding_pair(&text)
//    );
//}

// implementing this expected behavior elsewhere. no longer needed here...
//TODO: not currently working with opening/closing pairs that are the same(', ", etc.)
//#[test] fn with_same_surrounding_pair_opening_and_closing(){
//    //idk"some""t"h"i"n"g""else"
//    let text = Rope::from("idk\"some\"\"t\"h\"i\"n\"g\"\"else");
//    let selection = Selection::new(Range::new(12, 13), Direction::Forward);
//    assert_eq!(
//        vec![
//            Selection::new(Range::new(11, 12), Direction::Forward),
//            //Selection::new(Range::new(17, 18), Direction::Forward)
//            Selection::new(Range::new(13, 14), Direction::Forward)
//        ],
//        selection.nearest_surrounding_pair(&text)
//    );
//}
