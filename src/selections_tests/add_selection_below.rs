//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//use crate::selections::Selections;
//
//// add selection below
//// 0    4          13    17
////"idk\nsomething\nelse\n"
////idk
////something
////else
////
//    // within lines of same len or more
//        // non extended
//            //bar
//            #[test] fn add_selection_below_with_no_selection_extension_bar_semantics(){
//                let text = Rope::from("idk\nsomething\nelse\n");
//                assert_eq!(
//                    //Ok(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 0, &text)),
//                    Ok(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text)),
//                    //Selections::new(vec![Selection::new(0, 0)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                );
//            }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(0, 1)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(1, 0), Selection::new(5, 4)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Backward), Selection::new(Range::new(4, 5), Direction::Backward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(1, 0)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//        // extended
//            //bar
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(1, 3), Selection::new(5, 7)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Forward), Selection::new(Range::new(5, 7), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(1, 3)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 1), Selection::new(7, 5)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Backward), Selection::new(Range::new(5, 7), Direction::Backward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(3, 1)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    );
//                }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(1, 3), Selection::new(5, 7)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Forward), Selection::new(Range::new(5, 7), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(1, 3)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 1), Selection::new(7, 5)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Backward), Selection::new(Range::new(5, 7), Direction::Backward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(3, 1)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//    // adding to longer line
//        // non extended
//            //bar
//            #[test] fn add_selection_below_to_longer_line_with_no_selection_extension_bar_semantics(){
//                let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                assert_eq!(
//                    //Ok(Selections::new(vec![Selection::new(8, 8), Selection::new(18, 18)], 0, &text)),
//                    Ok(Selections::new(vec![Selection::new(Range::new(8, 8), Direction::Forward), Selection::new(Range::new(18, 18), Direction::Forward)], 0, &text)),
//                    //Selections::new(vec![Selection::new(8, 8)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    Selections::new(vec![Selection::new(Range::new(8, 8), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                );
//            }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_to_longer_line_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(8, 9), Selection::new(18, 19)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(8, 9), Direction::Forward), Selection::new(Range::new(18, 19), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(8, 9)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(8, 9), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_to_longer_line_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(9, 8), Selection::new(19, 18)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(8, 9), Direction::Backward), Selection::new(Range::new(18, 19), Direction::Backward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(9, 8)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(8, 9), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//        // extended
//            //bar
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_to_longer_line_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 9), Selection::new(14, 19)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Forward), Selection::new(Range::new(14, 19), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(4, 9)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_to_longer_line_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(9, 4), Selection::new(19, 14)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Backward), Selection::new(Range::new(14, 19), Direction::Backward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(9, 4)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    );
//                }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_to_longer_line_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 9), Selection::new(14, 19)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Forward), Selection::new(Range::new(14, 19), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(4, 9)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_to_longer_line_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(9, 4), Selection::new(19, 14)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Backward), Selection::new(Range::new(14, 19), Direction::Backward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(9, 4)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//    // adding to shorter line   //TODO: maybe add another newline at end, to make sure we aren't unintentionally testing any end of file functionality
//        // non extended
//            //bar
//            #[test] fn add_selection_below_to_shorter_line_with_no_selection_extension_bar_semantics(){
//                let text = Rope::from("idk\nsomething\nelse\n");
//                assert_eq!(
//                    //Ok(Selections::new(vec![Selection::new(13, 13), Selection::new(18, 18)], 0, &text)),
//                    Ok(Selections::new(vec![Selection::new(Range::new(13, 13), Direction::Forward), Selection::new(Range::new(18, 18), Direction::Forward)], 0, &text)),
//                    //Selections::new(vec![Selection::new(13, 13)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    Selections::new(vec![Selection::new(Range::new(13, 13), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                );
//            }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_to_shorter_line_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(13, 14), Selection::new(18, 19)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(13, 14), Direction::Forward), Selection::new(Range::new(18, 19), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(13, 14)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(13, 14), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Backward
//                #[test] fn add_backward_selection_below_to_shorter_line_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(14, 13), Selection::new(19, 18)], 0, &text)),                        // i d k \n s o m e t h i n g \n e l s e<\n|
//                        Ok(Selections::new(vec![Selection::new(Range::new(13, 14), Direction::Backward), Selection::new(Range::new(18, 19), Direction::Backward)], 0, &text)),                        // i d k \n s o m e t h i n g \n e l s e<\n|
//                        //Selections::new(vec![Selection::new(14, 13)], 0, &text).add_selection_below(&text, CursorSemantics::Block)  // i d k \n s o m e t h i n g<\n|e l s e \n
//                        Selections::new(vec![Selection::new(Range::new(13, 14), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)  // i d k \n s o m e t h i n g<\n|e l s e \n
//                    );
//                }
//        // extended
//            //bar
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_to_shorter_line_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 13), Selection::new(14, 19)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Forward), Selection::new(Range::new(14, 19), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(4, 13)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_to_shorter_line_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(13, 4), Selection::new(19, 14)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Backward), Selection::new(Range::new(14, 19), Direction::Backward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(13, 4)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    );
//                }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_to_shorter_line_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 13), Selection::new(14, 19)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Forward), Selection::new(Range::new(14, 19), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(4, 13)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_to_shorter_line_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(13, 4), Selection::new(19, 14)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Backward), Selection::new(Range::new(14, 19), Direction::Backward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(13, 4)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//    // to empty line
//        // non extended
//            //bar
//            #[test] fn add_selection_below_to_empty_line_no_selection_extension_bar_semantics(){
//                let text = Rope::from("idk\n");
//                assert_eq!(
//                    //Ok(Selections::new(vec![Selection::new(3, 3), Selection::new(4, 4)], 0, &text)),
//                    Ok(Selections::new(vec![Selection::new(Range::new(3, 3), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text)),
//                    //Selections::new(vec![Selection::new(3, 3)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    Selections::new(vec![Selection::new(Range::new(3, 3), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                );
//            }
//            //block
//                //selection direction forward
//                #[test] fn add_forward_selection_below_to_empty_line_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 4), Selection::new(4, 5)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(3, 4)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction backward
//                #[test] fn add_backward_selection_below_to_empty_line_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 3), Selection::new(5, 4)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Backward), Selection::new(Range::new(4, 5), Direction::Backward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(4, 3)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//        // extended
//            //bar
//                //selection direction forward
//                #[test] fn add_forward_selection_below_to_empty_line_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 4)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(0, 3)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    );
//                }
//                //selection direction backward
//                #[test] fn add_backward_selection_below_to_empty_line_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\n"); //<idk|\n
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 0), Selection::new(4, 4)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Backward), Selection::new(Range::new(4, 4), Direction::Backward/*to match primary direction*/)], 0, &text)),
//                        //Selections::new(vec![Selection::new(3, 0)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)
//                    );
//                }
//            //block
//                //selection direction forward
//                #[test] fn add_forward_selection_below_to_empty_line_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 5)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(0, 3)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction backward
//                #[test] fn add_backward_selection_below_to_empty_line_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 0), Selection::new(5, 4)], 0, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Backward), Selection::new(Range::new(4, 5), Direction::Backward)], 0, &text)),
//                        //Selections::new(vec![Selection::new(3, 0)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)
//                    );
//                }
//    // to line with only newline char
//        // non extended
//            //bar
//            #[test] fn add_selection_below_to_line_with_newline_no_selection_extension_bar_semantics(){
//                let text = Rope::from("idk\n\n");
//                assert_eq!(
//                    //Ok(Selections::new(vec![Selection::new(3, 3), Selection::new(4, 4)], 0, &text)),                        // i d k \n|\n
//                    Ok(Selections::new(vec![Selection::new(Range::new(3, 3), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text)),                        // i d k \n|\n
//                    //Selections::new(vec![Selection::new(3, 3)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)  // i d k|\n \n
//                    Selections::new(vec![Selection::new(Range::new(3, 3), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)  // i d k|\n \n
//                );
//            }
//            //block
//                //selection direction forward
//                #[test] fn add_forward_selection_below_to_line_with_newline_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\n\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 4), Selection::new(4, 5)], 0, &text)),                            // i d k \n|\n>
//                        Ok(Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text)),                            // i d k \n|\n>
//                        //Selections::new(vec![Selection::new(3, 4)], 0, &text).add_selection_below(&text, CursorSemantics::Block)    // i d k|\n>\n
//                        Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)    // i d k|\n>\n
//                    );
//                }
//                //selection direction backward
//                #[test] fn add_backward_selection_below_to_line_with_newline_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\n\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 3), Selection::new(5, 4)], 0, &text)),                            // i d k \n<\n|
//                        Ok(Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Backward), Selection::new(Range::new(4, 5), Direction::Backward)], 0, &text)),                            // i d k \n<\n|
//                        //Selections::new(vec![Selection::new(4, 3)], 0, &text).add_selection_below(&text, CursorSemantics::Block)    // i d k<\n|\n
//                        Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)    // i d k<\n|\n
//                    );
//                }
//        // extended
//            //bar
//                //selection direction forward
//                #[test] fn add_forward_selection_below_to_line_with_newline_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\n\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 4)], 0, &text)),                        // i d k \n|\n
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text)),                        // i d k \n|\n
//                        //Selections::new(vec![Selection::new(0, 3)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)  //|i d k>\n \n
//                        Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)  //|i d k>\n \n
//                    );
//                }
//                //selection direction backward
//                #[test] fn add_backward_selection_below_to_line_with_newline_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\n\n");   //<idk|\n\n
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 0), Selection::new(4, 4)], 0, &text)),                        // i d k \n|\n
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Backward), Selection::new(Range::new(4, 4), Direction::Backward/*to match primary direction*/)], 0, &text)),                        // i d k \n|\n
//                        //Selections::new(vec![Selection::new(3, 0)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)  //<i d k|\n \n
//                        Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar)  //<i d k|\n \n
//                    );
//                }
//            //block
//                //selection direction forward
//                #[test] fn add_forward_selection_below_to_line_with_newline_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\n\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 3), Selection::new(4, 5)], 0, &text)),                            // i d k \n|\n>
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text)),                            // i d k \n|\n>
//                        //Selections::new(vec![Selection::new(0, 3)], 0, &text).add_selection_below(&text, CursorSemantics::Block)    //|i d k>\n \n
//                        Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)    //|i d k>\n \n
//                    );
//                }
//                //selection direction backward
//                #[test] fn add_backward_selection_below_to_line_with_newline_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\n\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 0), Selection::new(5, 4)], 0, &text)),                            // i d k \n<\n|
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Backward), Selection::new(Range::new(4, 5), Direction::Backward)], 0, &text)),                            // i d k \n<\n|
//                        //Selections::new(vec![Selection::new(3, 0)], 0, &text).add_selection_below(&text, CursorSemantics::Block)    //<i d k|\n \n
//                        Selections::new(vec![Selection::new(Range::new(0, 3), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block)    //<i d k|\n \n
//                    );
//                }
//    // should error if on bottom line
//        // non extended
//            //bar
//            #[test] fn add_selection_below_should_error_when_selection_on_last_line_bar_semantics(){
//                let text = Rope::from("idk\nsomething\nelse\n");
//                //assert!(Selections::new(vec![Selection::new(19, 19)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                assert!(Selections::new(vec![Selection::new(Range::new(19, 19), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//            }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_should_error_when_selection_on_last_line_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(19, 20)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(19, 20), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                }
//                //selection direction Bacward   //added check to ensure last selection's end is not > doc text. should prevent panic
//                #[test] fn add_backward_selection_below_should_error_when_selection_on_last_line_block_semantics(){
//                    // i d k
//                    // s o m e t h i n g
//                    // e l s e
//                    //< |
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(20, 19)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(19, 20), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                }
//        // extended
//            //bar
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_should_error_when_extended_selection_on_last_line_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\nidk");
//                    //assert!(Selections::new(vec![Selection::new(19, 22)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(19, 22), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_should_error_when_extended_selection_on_last_line_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\nidk");
//                    //assert!(Selections::new(vec![Selection::new(22, 19)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(19, 22), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_should_error_when_extended_selection_on_last_line_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\nidk");
//                    //assert!(Selections::new(vec![Selection::new(19, 22)], 0, &text).add_selection_below(&text, CursorSemantics::Block).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(19, 22), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block).is_err());
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_should_error_when_extended_selection_on_last_line_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\nidk");
//                    //assert!(Selections::new(vec![Selection::new(22, 19)], 0, &text).add_selection_below(&text, CursorSemantics::Block).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(19, 22), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block).is_err());
//                }
//    // should error if any selection is multiline
//            //bar
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_should_error_when_any_selection_is_multi_line_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(4, 15)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(4, 15), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_should_error_when_any_selection_is_multi_line_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(15, 4)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(4, 15), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Bar).is_err());
//                }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_below_should_error_when_any_selection_is_multi_line_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(4, 15)], 0, &text).add_selection_below(&text, CursorSemantics::Block).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(4, 15), Direction::Forward)], 0, &text).add_selection_below(&text, CursorSemantics::Block).is_err());
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_below_should_error_when_any_selection_is_multi_line_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(15, 4)], 0, &text).add_selection_below(&text, CursorSemantics::Block).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(4, 15), Direction::Backward)], 0, &text).add_selection_below(&text, CursorSemantics::Block).is_err());
//                }
//