use ropey::Rope;
use crate::text_util;

#[test] fn idk(){
    let text = Rope::from("fn idk(){/*something*/}");
    assert_eq!(22, text_util::previous_word_boundary(23, &text)); // f n   i d k ( ) { / * s o m e t h i n g * / }|  // f n   i d k ( ) { / * s o m e t h i n g * /|}
    assert_eq!(21, text_util::previous_word_boundary(22, &text)); // f n   i d k ( ) { / * s o m e t h i n g * /|}   // f n   i d k ( ) { / * s o m e t h i n g *|/ }
    assert_eq!(20, text_util::previous_word_boundary(21, &text)); // f n   i d k ( ) { / * s o m e t h i n g *|/ }   // f n   i d k ( ) { / * s o m e t h i n g|* / }
    assert_eq!(11, text_util::previous_word_boundary(20, &text)); // f n   i d k ( ) { / * s o m e t h i n g|* / }   // f n   i d k ( ) { / *|s o m e t h i n g * / }
    assert_eq!(10, text_util::previous_word_boundary(11, &text)); // f n   i d k ( ) { / *|s o m e t h i n g * / }   // f n   i d k ( ) { /|* s o m e t h i n g * / }
    assert_eq!(9, text_util::previous_word_boundary(10, &text));  // f n   i d k ( ) { /|* s o m e t h i n g * / }   // f n   i d k ( ) {|/ * s o m e t h i n g * / }
    assert_eq!(8, text_util::previous_word_boundary(9, &text));   // f n   i d k ( ) {|/ * s o m e t h i n g * / }   // f n   i d k ( )|{ / * s o m e t h i n g * / }
    assert_eq!(7, text_util::previous_word_boundary(8, &text));   // f n   i d k ( )|{ / * s o m e t h i n g * / }   // f n   i d k (|) { / * s o m e t h i n g * / }
    assert_eq!(6, text_util::previous_word_boundary(7, &text));   // f n   i d k (|) { / * s o m e t h i n g * / }   // f n   i d k|( ) { / * s o m e t h i n g * / }
    assert_eq!(3, text_util::previous_word_boundary(6, &text));   // f n   i d k|( ) { / * s o m e t h i n g * / }   // f n  |i d k ( ) { / * s o m e t h i n g * / }
    assert_eq!(0, text_util::previous_word_boundary(3, &text));   // f n  |i d k ( ) { / * s o m e t h i n g * / }   //|f n   i d k ( ) { / * s o m e t h i n g * / }
    
    let text = Rope::from(" idk");
    assert_eq!(0, text_util::previous_word_boundary(1, &text));
    let text = Rope::from("\tidk");
    assert_eq!(0, text_util::previous_word_boundary(1, &text));
    let text = Rope::from("\nidk");
    assert_eq!(0, text_util::previous_word_boundary(1, &text));
}
