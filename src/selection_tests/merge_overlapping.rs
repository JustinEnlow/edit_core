use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

// note: only considering block semantics currently
// this is only for overlapping merges, not indiscriminate merges       //maybe this should be in selections_tests/merge_overlapping.rs
    // non extended cursors should always be Direction::Forward when moved. but we should still test mixed directions
    // can only overlap when primary or other are stationary

    // non extended multicursors
        #[test] fn with_non_extended_multicursors_with_self_and_other_forward(){
            let text = Rope::from("idk\n");
            //cases covered:
                //primary and other forward             stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i d k|\n>    //primary
                    // i d|k>\n     //other
                    // i d k|\n>    //primary stationary
                    // i d k|\n>    //other moved forward

                //primary and other forward             stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i d|k>\n     //primary
                    // i d k|\n>    //other
                    // i d k|\n>    //primary moved forward
                    // i d k|\n>    //other stationary

                //primary and other forward             stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    //|i>d k \n   //primary
                    // i|d>k \n   //other
                    //|i>d k \n   //primary stationary
                    //|i>d k \n   //other moved backward

                //primary and other forward             stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i|d>k \n     //primary
                    //|i>d k \n     //other
                    //|i>d k \n     //primary moved backward
                    //|i>d k \n     //other stationary
            assert_eq!(Selection::with_stored_line_position(3, 4, 3), Selection::new(3, 4).merge_overlapping(&Selection::new(3, 4), &text, CursorSemantics::Block).unwrap());
            assert_eq!(Selection::with_stored_line_position(0, 1, 0), Selection::new(0, 1).merge_overlapping(&Selection::new(1, 0), &text, CursorSemantics::Block).unwrap());
        }
        #[test] fn with_non_extended_multicursors_with_self_and_other_backward(){
            let text = Rope::from("idk\n");
            //cases covered:
                //primary and other backward            stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i d k<\n|    //primary
                    // i d<k|\n     //other
                    // i d k<\n|    //primary stationary
                    // i d k<\n|    //other moved forward

                //primary and other backward            stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i d<k|\n     //primary
                    // i d k<\n|    //other
                    // i d k<\n|    //primary moved forward
                    // i d k<\n|    //other stationary

                //primary and other backward            stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    //<i|d k \n   //primary
                    // i<d|k \n   //other
                    //<i|d k \n   //primary stationary
                    //<i|d k \n   //other moved backward

                //primary and other backward            stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i<d|k \n     //primary
                    //<i|d k \n     //other
                    //<i|d k \n     //primary moved backward
                    //<i|d k \n     //other stationary
            assert_eq!(Selection::with_stored_line_position(4, 3, 3), Selection::new(4, 3).merge_overlapping(&Selection::new(4, 3), &text, CursorSemantics::Block).unwrap());
        }
        #[test] fn with_non_extended_multicursors_with_self_forward_and_other_backward(){
            let text = Rope::from("idk\n");
            //cases covered:
                //primary forward, other backward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i d k|\n>    //primary
                    // i d<k|\n     //other
                    // i d k|\n>    //primary stationary
                    // i d k<\n|    //other moved forward

                //primary forward, other backward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i d|k>\n     //primary
                    // i d k<\n|    //other
                    // i d k|\n>    //primary moved forward
                    // i d k<\n|    //other stationary

                //primary forward, other backward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    //|i>d k \n   //primary
                    // i<d|k \n   //other
                    //|i>d k \n   //primary stationary
                    //<i|d k \n   //other moved backward

                //primary forward, other backward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i|d>k \n     //primary
                    //<i|d k \n     //other
                    //|i>d k \n     //primary moved backward
                    //<i|d k \n     //other stationary
            assert_eq!(Selection::with_stored_line_position(3, 4, 3), Selection::new(3, 4).merge_overlapping(&Selection::new(4, 3), &text, CursorSemantics::Block).unwrap());
        }
        #[test] fn with_non_extended_multicursors_with_self_backward_and_other_forward(){
            let text = Rope::from("idk\n");
            //cases covered:
                //primary backward, other forward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i d k<\n|    //primary
                    // i d|k>\n     //other
                    // i d k<\n|    //primary stationary
                    // i d k|\n>    //other moved forward

                //primary backward, other forward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i d<k|\n     //primary
                    // i d k|\n>    //other
                    // i d k<\n|    //primary moved forward
                    // i d k|\n>    //other stationary

                //primary backward, other forward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    //<i|d k \n   //primary
                    // i|d>k \n   //other
                    //<i|d k \n   //primary stationary
                    //|i>d k \n   //other moved backward

                //primary backward, other forward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
                    // i<d|k \n     //primary
                    //|i>d k \n     //other
                    //<i|d k \n     //primary moved backward
                    //|i>d k \n     //other stationary
            assert_eq!(Selection::with_stored_line_position(3, 4, 3), Selection::new(4, 3).merge_overlapping(&Selection::new(3, 4), &text, CursorSemantics::Block).unwrap());
        }
//    // non extended multicursors move forward til overlapping
//        //primary stationary, while other cursor continues to move forward
//            //match direction
//                //primary and other forward             stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i d k|\n>    //primary
//                    // i d|k>\n     //other
//                    // i d k|\n>    //primary stationary
//                    // i d k|\n>    //other moved forward
//                
//                //primary and other backward            stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i d k<\n|    //primary
//                    // i d<k|\n     //other
//                    // i d k<\n|    //primary stationary
//                    // i d k<\n|    //other moved forward
//            
//            //mismatch direction
//                //primary forward, other backward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i d k|\n>    //primary
//                    // i d<k|\n     //other
//                    // i d k|\n>    //primary stationary
//                    // i d k<\n|    //other moved forward
//                
//                //primary backward, other forward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i d k<\n|    //primary
//                    // i d|k>\n     //other
//                    // i d k<\n|    //primary stationary
//                    // i d k|\n>    //other moved forward
//        
//        //other cursor stationary, while primary continues to move forward
//            //match direction
//                //primary and other forward             stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i d|k>\n     //primary
//                    // i d k|\n>    //other
//                    // i d k|\n>    //primary moved forward
//                    // i d k|\n>    //other stationary
//                //test case already covered by previous test...
//                
//                //primary and other backward            stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i d<k|\n     //primary
//                    // i d k<\n|    //other
//                    // i d k<\n|    //primary moved forward
//                    // i d k<\n|    //other stationary
//                //test case already covered by previous test...
//            
//            //mismatch direction
//                //primary forward, other backward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i d|k>\n     //primary
//                    // i d k<\n|    //other
//                    // i d k|\n>    //primary moved forward
//                    // i d k<\n|    //other stationary
//                //test case already covered by previous test...
//                
//                //primary backward, other forward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i d<k|\n     //primary
//                    // i d k|\n>    //other
//                    // i d k<\n|    //primary moved forward
//                    // i d k|\n>    //other stationary
//                //test case already covered by previous test...
//    
//    // non extended multicursors move backward til overlapping
//        //primary stationary, while other cursor continues to move backward
//            //match direction
//                //primary and other forward             stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    //|i>d k \n   //primary
//                    // i|d>k \n   //other
//                    //|i>d k \n   //primary stationary
//                    //|i>d k \n   //other moved backward
//                
//                //primary and other backward            stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    //<i|d k \n   //primary
//                    // i<d|k \n   //other
//                    //<i|d k \n   //primary stationary
//                    //<i|d k \n   //other moved backward
//            
//            //mismatch direction
//                //primary forward, other backward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    //|i>d k \n   //primary
//                    // i<d|k \n   //other
//                    //|i>d k \n   //primary stationary
//                    //<i|d k \n   //other moved backward
//                
//                //primary backward, other forward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    //<i|d k \n   //primary
//                    // i|d>k \n   //other
//                    //<i|d k \n   //primary stationary
//                    //|i>d k \n   //other moved backward
//        
//        //other cursor stationary, while primary continues to move backward
//            //match direction
//                //primary and other forward             stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i|d>k \n     //primary
//                    //|i>d k \n     //other
//                    //|i>d k \n     //primary moved backward
//                    //|i>d k \n     //other stationary
//                
//                //primary and other backward            stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i<d|k \n     //primary
//                    //<i|d k \n     //other
//                    //<i|d k \n     //primary moved backward
//                    //<i|d k \n     //other stationary
//            
//            //mismatch direction
//                //primary forward, other backward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i|d>k \n     //primary
//                    //<i|d k \n     //other
//                    //|i>d k \n     //primary moved backward
//                    //<i|d k \n     //other stationary
//                
//                //primary backward, other forward       stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
//                    // i<d|k \n     //primary
//                    //|i>d k \n     //other
//                    //<i|d k \n     //primary moved backward
//                    //|i>d k \n     //other stationary
//        
//        //neither can overlap until primary or other is forced to be stationary
    
    // mixed extended and non extended
        // primary forward non extended, other forward extended
        // primary backward non extended, other backward extended
        // primary forward non extended, other backward extended
        // primary backward non extended, other forward extended

        // primary forward extended, other forward non extended
        // primary backward extended, other backward non extended
        // primary forward extended, other backward non extended
        // primary backward extended, other forward non extended
    
//    // extended multicursors extend forward til overlapping
//        //primary stationary, while other selection continues to extend forward
//            //matched direction
//                //primary and other forward             stored_line_position after extension completion can be self.start, self.cursor, other.cursor
//                    // i d k|\n>  //primary
//                    // i d|k>\n   //other
//                    // i d k|\n>  //primary stationary
//                    // i d|k \n>  //other extended forward
//                
//                //primary and other backward            stored_line_position after extension completion can be self.start, self.cursor, other.cursor
//                    // i d k<\n|  //primary
//                    // i d<k|\n   //other
//                    // i d k<\n|  //primary stationary
//                    // i d|k \n>  //other extended forward          //note: selection direction changes here, due to how extension is implemented
//            
//            //mismatched direction
//                //primary forward, other backward       stored_line_position after extension completion can be self.start, self.cursor, other.cursor
//                    // i d k|\n>  //primary
//                    // i d<k|\n   //other
//                    // i d k|\n>  //primary stationary
//                    // i d|k \n>  //other extended forward          //note: selection direction changes here, due to how extension is implemented
//                
//                //primary backward, other forward       stored_line_position after extension completion can be self.start, self.cursor, other.cursor
//                    // i d k<\n|  //primary
//                    // i d|k>\n   //other
//                    // i d k<\n|  //primary stationary
//                    // i d|k \n>  //other extended forward
//        
//        //other cursor stationary, while primary continues to extend forward
//            //matched direction
//                //primary and other forward             stored_line_position after extension completion can be self.cursor, other.start, other.cursor
//                    // i d|k>\n   //primary
//                    // i d k|\n>  //other
//                    // i d|k \n>  //primary extended forward
//                    // i d k|\n>  //other stationary
//                
//                //primary and other backward            stored_line_position after extension completion can be self.cursor, other.start, other.cursor
//                    // i d<k|\n   //primary
//                    // i d k<\n|  //other
//                    // i d|k \n>  //primary extended forward        //note: selection direction changes here, due to how extension is implemented
//                    // i d k<\n|  //other stationary
//            
//            //mismatched direction
//                //primary forward, other backward       stored_line_position after extension completion can be self.cursor, other.start, other.cursor
//                    // i d|k>\n   //primary
//                    // i d k<\n|  //other
//                    // i d|k \n>  //primary extended forward
//                    // i d k<\n|  //extended stationary
//                
//                //primary backward, other forward       stored_line_position after extension completion can be self.cursor, other.start, other.cursor
//                    // i d<k|\n     //primary
//                    // i d k|\n>    //other
//                    // i d|k \n>    //primary extended forward      //note: selection direction changes here, due to how extension is implemented
//                    // i d k|\n>    //other stationary
//        
//        //neither stationary, extending forward
//            //matched direction
//                //primary and other forward             stored_line_position after extension completion can be self.cursor, other.end
//                    // i d|k>\n     //primary
//                    // i|d>k \n     //other
//                    // i d|k \n>    //primary extended forward
//                    // i|d k>\n     //other extended forward
//                
//                //primary and other backward            stored_line_position after extension completion can be self.cursor, other.end
//                    // i d<k|\n     //primary
//                    // i<d|k \n     //other
//                    // i d|k \n>    //primary extended forward      //note: selection direction changes here, due to how extension is implemented
//                    // i|d k>\n     //other extended forward        //note: selection direction changes here, due to how extension is implemented
//            
//            //mismatched direction
//                //primary forward, other backward       stored_line_position after extension completion can be self.cursor, other.end
//                    // i d|k>\n     //primary
//                    // i<d|k \n     //other
//                    // i d|k \n>    //primary extended forward
//                    // i|d k>\n     //other extended forward        //note: selection direction changes here, due to how extension is implemeted
//                
//                //primary backward, other forward       stored_line_position after extension completion can be self.cursor, other.end
//                    // i d<k|\n     //primary
//                    // i|d>k \n     //other
//                    // i d|k \n>    //primary extended forward      //note: selection direction changes here, due to how extension is implemented
//                    // i|d k>\n     //other extended forward
//    
//    // extended multicursors extend backward til overlapping
//        //primary stationary, while other selection continues to extend backward
//            //matched direction
//                //primary and other forward             stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
//                    //|i>d k \n     //primary
//                    // i|d>k \n     //other
//                    //|i>d k \n     //primary stationary
//                    //<i d|k \n     //other extended backward       //note: selection direction changes here, due to how extension is implemented
//                
//                //primary and other backward            stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
//                    //<i|d k \n     //primary
//                    // i<d|k \n     //other
//                    //<i|d k \n     //primary stationary
//                    //<i d|k \n     //other extended backward
//            
//            //mismatched direction
//                //primary forward, other backward       stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
//                    //|i>d k \n     //primary
//                    // i<d|k \n     //other
//                    //|i>d k \n     //primary stationary
//                    //<i d|k \n     //other extended backward
//                
//                //primary backward, other forward       stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
//                    //<i|d k \n     //primary
//                    // i|d>k \n     //other
//                    //<i|d k \n     //primary stationary
//                    //<i d|k \n     //other extended backward       //note: selection direction changes here, due to how extension is implemented
//        
//        //other cursor stationary, while primary continues to extend backward
//            //matched direction
//                //primary and other forward             stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
//                    // i|d>k \n     //primary
//                    //|i>d k \n     //other
//                    //<i d|k \n     //primary extended backward     //note: selection direction changes here, due to how extension is implemented
//                    //|i>d k \n     //other stationary
//                
//                //primary and other backward            stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
//                    // i<d|k \n     //primary
//                    //<i|d k \n     //other
//                    //<i d|k \n     //primary extended backward
//                    //<i|d k \n     //other stationary
//            
//            //mismatched direction
//                //primary forward, other backward       stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
//                    // i|d>k \n     //primary
//                    //<i|d k \n     //other
//                    //<i d|k \n     //primary extended backward     //note: selection direction changes here, due to how extension is implemented
//                    //<i|d k \n     //other stationary
//                
//                //primary backward, other forward       stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
//                    // i<d|k \n     //primary
//                    //|i>d k \n     //other
//                    //<i d|k \n     //primary extended backward
//                    //|i>d k \n     //other stationary
//        
//        //neither stationary, extending backward
//            //matched direction
//                //primary and other forward             stored_line_position after extension completion can be self.start, self.cursor
//                    // i|d>k \n     //primary
//                    // i d|k>\n     //other
//                    //<i d|k \n     //primary extended backward     //note: selection direction changes here, due to how extension is implemented
//                    // i<d k|\n     //other extended backward       //note: selection direction changes here, due to how extension is implemented
//                
//                //primary and other backward            stored_line_position after extension completion can be self.start, self.cursor
//                    // i<d|k \n     //primary
//                    // i d<k|\n     //other
//                    //<i d|k \n     //primary extended backward
//                    // i<d k|\n     //other extended backward
//            
//            //mismatched direction
//                //primary forward, other backward       stored_line_position after extension completion can be self.start, self.cursor
//                    // i|d>k \n     //primary
//                    // i d<k|\n     //other
//                    //<i d|k \n     //primary extended backward     //note: selection direction changes here, due to how extension is implemented
//                    // i<d k|\n     //other extended backward
//                
//                //primary backward, other forward       stored_line_position after extension completion can be self.start, self.cursor
//                    // i<d|k \n     //primary
//                    // i d|k>\n     //other
//                    //<i d|k \n     //primary extended backward
//                    // i<d k|\n     //other extended backward       //note: selection direction changes here, due to how extension is implemented