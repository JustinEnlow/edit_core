//use ropey::Rope;
//use crate::range::Range;
//use crate::selection::{Selection, CursorSemantics, Direction};
//use crate::selections::Selections;
//
//// add selection above
//// 0    4          13    17
////"idk\nsomething\nelse\n"
//    // within lines of same len
//        // non extended
//            //bar
//            #[test] fn add_selection_above_with_no_selection_extension_bar_semantics(){
//                let text = Rope::from("idk\nsomething\nelse\n");
//                assert_eq!(
//                    //Ok(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text)),
//                    Ok(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 1, &text)),
//                    //Selections::new(vec![Selection::new(4, 4)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    Selections::new(vec![Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                );
//            }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(4, 5)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(1, 0), Selection::new(5, 4)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Backward), Selection::new(Range::new(4, 5), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(5, 4)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(4, 5), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//        // extended
//            //bar
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(1, 3), Selection::new(5, 7)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Forward), Selection::new(Range::new(5, 7), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(5, 7)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(5, 7), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 1), Selection::new(7, 5)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Backward), Selection::new(Range::new(5, 7), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(7, 5)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(5, 7), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    );
//                }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(1, 3), Selection::new(5, 7)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Forward), Selection::new(Range::new(5, 7), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(5, 7)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(5, 7), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 1), Selection::new(7, 5)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Backward), Selection::new(Range::new(5, 7), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(7, 5)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(5, 7), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//    // adding to longer line
//        // non extended
//            //bar
//            #[test] fn add_selection_above_to_longer_line_with_no_selection_extension_bar_semantics(){
//                let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                assert_eq!(
//                    //Ok(Selections::new(vec![Selection::new(8, 8), Selection::new(18, 18)], 1, &text)),
//                    Ok(Selections::new(vec![Selection::new(Range::new(8, 8), Direction::Forward), Selection::new(Range::new(18, 18), Direction::Forward)], 1, &text)),
//                    //Selections::new(vec![Selection::new(18, 18)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    Selections::new(vec![Selection::new(Range::new(18, 18), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                );
//            }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_to_longer_line_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(8, 9), Selection::new(18, 19)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(8, 9), Direction::Forward), Selection::new(Range::new(18, 19), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(18, 19)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(18, 19), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_to_longer_line_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(9, 8), Selection::new(19, 18)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(8, 9), Direction::Backward), Selection::new(Range::new(18, 19), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(19, 18)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(18, 19), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//        // extended
//            //bar
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_to_longer_line_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 9), Selection::new(14, 19)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Forward), Selection::new(Range::new(14, 19), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(14, 19)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(14, 19), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_to_longer_line_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(9, 4), Selection::new(19, 14)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Backward), Selection::new(Range::new(14, 19), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(19, 14)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(14, 19), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    );
//                }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_to_longer_line_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 9), Selection::new(14, 19)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Forward), Selection::new(Range::new(14, 19), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(14, 19)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(14, 19), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_to_longer_line_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //len 19
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(9, 4), Selection::new(19, 14)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(4, 9), Direction::Backward), Selection::new(Range::new(14, 19), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(19, 14)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(14, 19), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//    // adding to shorter line
//        // non extended
//            //bar
//            #[test] fn add_selection_above_to_shorter_line_with_no_selection_extension_bar_semantics(){
//                let text = Rope::from("idk\nsomething\nelse\n");
//                assert_eq!(
//                    //Ok(Selections::new(vec![Selection::new(3, 3), Selection::new(13, 13)], 1, &text)),
//                    Ok(Selections::new(vec![Selection::new(Range::new(3, 3), Direction::Forward), Selection::new(Range::new(13, 13), Direction::Forward)], 1, &text)),
//                    //Selections::new(vec![Selection::new(13, 13)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    Selections::new(vec![Selection::new(Range::new(13, 13), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                );
//            }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_to_shorter_line_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(3, 4), Selection::new(13, 14)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Forward), Selection::new(Range::new(13, 14), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(13, 14)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(13, 14), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_to_shorter_line_with_no_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 3), Selection::new(14, 13)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(3, 4), Direction::Backward), Selection::new(Range::new(13, 14), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(14, 13)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(13, 14), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//        // extended
//            //bar
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_to_shorter_line_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 4), Selection::new(4, 13)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 4), Direction::Forward), Selection::new(Range::new(4, 13), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(4, 13)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_to_shorter_linepwith_selection_extension_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 0), Selection::new(13, 4)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 4), Direction::Backward), Selection::new(Range::new(4, 13), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(13, 4)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    );
//                }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_to_shorter_line_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 4), Selection::new(4, 13)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 4), Direction::Forward), Selection::new(Range::new(4, 13), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(4, 13)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_to_shorter_line_with_selection_extension_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");    //idk\n<something|\nelse\n
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(4, 0), Selection::new(13, 4)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 4), Direction::Backward), Selection::new(Range::new(4, 13), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(13, 4)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(4, 13), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//    // to empty line(can't happen in add selection above. won't have an empty line followed by populated lines)
//        // non extended
//            //bar
//            //block
//                //selection direction forward
//                //selection direction backward
//        // extended
//            //bar
//                //selection direction forward
//                //selection direction backward
//            //block
//                //selection direction forward
//                //selection direction backward
//    // to line with only newline char
//        // non extended
//            //bar
//            #[test] fn add_selection_above_to_line_with_newline_no_selection_extension_bar_semantics(){
//                let text = Rope::from("\nidk\n");
//                assert_eq!(
//                    //Ok(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 4)], 1, &text)),
//                    Ok(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(4, 4), Direction::Forward)], 1, &text)),
//                    //Selections::new(vec![Selection::new(4, 4)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    Selections::new(vec![Selection::new(Range::new(4, 4), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                );
//            }
//            //block
//                //selection direction forward
//                #[test] fn add_forward_selection_above_to_line_with_newline_no_selection_extension_block_semantics(){
//                    let text = Rope::from("\nidk\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 1), Selection::new(4, 5)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(4, 5), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(4, 5)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(4, 5), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction backward
//                #[test] fn add_backward_selection_above_to_line_with_newline_no_selection_extension_block_semantics(){
//                    let text = Rope::from("\nidk\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(1, 0), Selection::new(5, 4)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Backward), Selection::new(Range::new(4, 5), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(5, 4)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(4, 5), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//        // extended
//            //bar
//                //selection direction forward
//                #[test] fn add_forward_selection_above_to_line_with_newline_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("\nidk\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 0), Selection::new(1, 4)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward), Selection::new(Range::new(1, 4), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(1, 4)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(1, 4), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    );
//                }
//                //selection direction backward
//                #[test] fn add_backward_selection_above_to_line_with_newline_with_selection_extension_bar_semantics(){
//                    let text = Rope::from("\nidk\n");   //\n<idk|\n
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 0), Selection::new(4, 1)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Backward/*to match primary direction*/), Selection::new(Range::new(1, 4), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(4, 1)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                        Selections::new(vec![Selection::new(Range::new(1, 4), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar)
//                    );
//                }
//            //block
//                //selection direction forward
//                #[test] fn add_forward_selection_above_to_line_with_newline_with_selection_extension_block_semantics(){
//                    let text = Rope::from("\nidk\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(0, 1), Selection::new(1, 4)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward), Selection::new(Range::new(1, 4), Direction::Forward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(1, 4)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(1, 4), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//                //selection direction backward
//                #[test] fn add_backward_selection_above_to_line_with_newline_with_selection_extension_block_semantics(){
//                    let text = Rope::from("\nidk\n");
//                    assert_eq!(
//                        //Ok(Selections::new(vec![Selection::new(1, 0), Selection::new(4, 1)], 1, &text)),
//                        Ok(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Backward), Selection::new(Range::new(1, 4), Direction::Backward)], 1, &text)),
//                        //Selections::new(vec![Selection::new(4, 1)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                        Selections::new(vec![Selection::new(Range::new(1, 4), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Block)
//                    );
//                }
//    // should error if on top line
//        // non extended
//            //bar
//            #[test] fn add_selection_above_should_error_when_selection_on_line_0_bar_semantics(){
//                let text = Rope::from("idk\nsomething\nelse\n");
//                //assert!(Selections::new(vec![Selection::new(0, 0)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                assert!(Selections::new(vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//            }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_should_error_when_selection_on_line_0_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(0, 1)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_should_error_when_selection_on_line_0_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(1, 0)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(0, 1), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                }
//        // extended
//            //bar
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_should_error_when_extended_selection_on_line_0_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(1, 3)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_should_error_when_extended_selection_on_line_0_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(3, 1)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_should_error_when_extended_selection_on_line_0_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(1, 3)], 0, &text).add_selection_above(&text, CursorSemantics::Block).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Block).is_err());
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_should_error_when_extended_selection_on_line_0_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(3, 1)], 0, &text).add_selection_above(&text, CursorSemantics::Block).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(1, 3), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Block).is_err());
//                }
//    // should error if any selection is multiline
//            //bar
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_should_error_when_any_selection_is_multi_line_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(4, 15)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(4, 15), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_should_error_when_any_selection_is_multi_line_bar_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(15, 4)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(4, 15), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Bar).is_err());
//                }
//            //block
//                //selection direction Forward
//                #[test] fn add_forward_selection_above_should_error_when_any_selection_is_multi_line_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(4, 15)], 0, &text).add_selection_above(&text, CursorSemantics::Block).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(4, 15), Direction::Forward)], 0, &text).add_selection_above(&text, CursorSemantics::Block).is_err());
//                }
//                //selection direction Bacward
//                #[test] fn add_backward_selection_above_should_error_when_any_selection_is_multi_line_block_semantics(){
//                    let text = Rope::from("idk\nsomething\nelse\n");
//                    //assert!(Selections::new(vec![Selection::new(15, 4)], 0, &text).add_selection_above(&text, CursorSemantics::Block).is_err());
//                    assert!(Selections::new(vec![Selection::new(Range::new(4, 15), Direction::Backward)], 0, &text).add_selection_above(&text, CursorSemantics::Block).is_err());
//                }
//