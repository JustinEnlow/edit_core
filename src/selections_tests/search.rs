//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, Direction};
//use crate::selections::Selections;
//
//#[test] fn standard_use_with_whole_text_selected(){
//    let text = Rope::from("idk\nsome\nshit\nand\nsome\nother\nshit\n"); //len 34
//    //let selections = Selections::new(vec![Selection::new(0, 34)], 0, &text);
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 34), Direction::Forward)], 0, &text);
//    //assert_eq!(Selections::new(vec![Selection::new(9, 13), Selection::new(29, 33)], 0, &text), selections.search("shit", &text).unwrap());
//    assert_eq!(Selections::new(vec![Selection::new(Range::new(9, 13), Direction::Forward), Selection::new(Range::new(29, 33), Direction::Forward)], 0, &text), selections.search("shit", &text).unwrap());
//}
//
//#[test] fn standard_use_with_multiple_selections(){
//    let text = Rope::from("idk\nsome\nshit\nidk\nsome\nshit\nidk\nsome\nshit\n");   //len 42
//    //let selections = Selections::new(vec![Selection::new(0, 14), Selection::new(14, 28), Selection::new(28, 42)], 0, &text);
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 14), Direction::Forward), Selection::new(Range::new(14, 28), Direction::Forward), Selection::new(Range::new(28, 42), Direction::Forward)], 0, &text);
//    //assert_eq!(Selections::new(vec![Selection::new(9, 13), Selection::new(23, 27), Selection::new(37, 41)], 0, &text), selections.search("shit", &text).unwrap());
//    assert_eq!(Selections::new(vec![Selection::new(Range::new(9, 13), Direction::Forward), Selection::new(Range::new(23, 27), Direction::Forward), Selection::new(Range::new(37, 41), Direction::Forward)], 0, &text), selections.search("shit", &text).unwrap());
//}
//#[test] fn when_primary_selection_is_last_in_a_multiselection_and_no_match_in_that_selection(){
//    let text = Rope::from("idk\nsome\nshit\nidk\nsome\nshit\nidk\nsome\ncrap\n");   //len 42
//    //let selections = Selections::new(vec![Selection::new(0, 14), Selection::new(14, 28), Selection::new(28, 42)], 2, &text);
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 14), Direction::Forward), Selection::new(Range::new(14, 28), Direction::Forward), Selection::new(Range::new(28, 42), Direction::Forward)], 2, &text);
//    //assert_eq!(Selections::new(vec![Selection::new(9, 13), Selection::new(23, 27)], 1, &text), selections.search("shit", &text).unwrap());
//    assert_eq!(Selections::new(vec![Selection::new(Range::new(9, 13), Direction::Forward), Selection::new(Range::new(23, 27), Direction::Forward)], 1, &text), selections.search("shit", &text).unwrap());
//}
//
////#[test] fn standard_use_with_no_selections_extended(){  //note: this is a future goal. ok to fail, for the moment...
////    let text = Rope::from("idk\nsome\nshit\nand\nsome\nother\nshit\n"); //len 34
////    //let selections = Selections::new(vec![Selection::new(0, 1)], 0, &text);
////    let selections = Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, &text);
////    //assert_eq!(Selections::new(vec![Selection::new(9, 13), Selection::new(29, 33)], 0, &text), selections.search("shit", &text).unwrap());
////    assert_eq!(Selections::new(vec![Selection::new(Range::new(9, 13), Direction::Forward), Selection::new(Range::new(29, 33), Direction::Forward)], 0, &text), selections.search("shit", &text).unwrap());
////}
//
//#[test] fn errors_if_no_matches(){
//    let text = Rope::from("idk\nsome\nshit\nand\nsome\nother\nshit\n"); //len 34
//    //let selections = Selections::new(vec![Selection::new(0, 34)], 0, &text);
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 34), Direction::Forward)], 0, &text);
//    assert!(selections.search("banana", &text).is_err());
//}
//
//#[test] fn errors_if_search_string_is_empty(){
//    let text = Rope::from("idk\nsome\nshit\nand\nsome\nother\nshit\n"); //len 34
//    //let selections = Selections::new(vec![Selection::new(0, 34)], 0, &text);
//    let selections = Selections::new(vec![Selection::new(Range::new(0, 34), Direction::Forward)], 0, &text);
//    assert!(selections.search("", &text).is_err());
//}
//