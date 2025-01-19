use ropey::Rope;
use crate::selection::{Selection, CursorSemantics};

    // non extended cursors should always be Direction::Forward when moved. but we should still test mixed directions
    // cursor movements can only overlap when primary or other are stationary(extensions can overlap any time)

// non extended multicursors        these cases result from multicursor movements where one cursor saturates at a doc boundary until another cursor overlaps it
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
        assert_eq!(Selection::with_stored_line_position(1, 0, 0), Selection::new(1, 0).merge_overlapping(&Selection::new(1, 0), &text, CursorSemantics::Block).unwrap());
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
        assert_eq!(Selection::with_stored_line_position(0, 1, 0), Selection::new(0, 1).merge_overlapping(&Selection::new(1, 0), &text, CursorSemantics::Block).unwrap());
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
        assert_eq!(Selection::with_stored_line_position(0, 1, 0), Selection::new(1, 0).merge_overlapping(&Selection::new(0, 1), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_non_extended_multicursors_bar_semantics(){
        let text = Rope::from("idk\n");
        //cases covered:                            stored_line_position after movement completion can be self.start, self.cursor, other.start, other.cursor
            // i d k \n|    //primary
            // i d k|\n     //other
            // i d k \n|    //primary stationary
            // i d k \n|    //other moved forward

            // i d k|\n     //primary
            // i d k \n|    //other
            // i d k \n|    //primary moved forward
            // i d k \n|    //other stationary

            //|i d k \n     //primary
            // i|d k \n     //other
            //|i d k \n     //primary stationary
            //|i d k \n     //other moved backward

            // i|d k \n     //primary
            //|i d k \n     //other
            //|i d k \n     //primary moved backward
            //|i d k \n     //other stationary
        assert_eq!(Selection::with_stored_line_position(4, 4, 0), Selection::new(4, 4).merge_overlapping(&Selection::new(4, 4), &text, CursorSemantics::Bar).unwrap());
        assert_eq!(Selection::with_stored_line_position(0, 0, 0), Selection::new(0, 0).merge_overlapping(&Selection::new(0, 0), &text, CursorSemantics::Bar).unwrap());
    }



// mixed extended and non extended      these cases result from multicursor movements where one cursor saturates at a doc boundary, while another selection extends until it overlaps it
    #[test] fn with_self_non_extended_forward_and_other_extended_forward(){
        let text = Rope::from("idk\n");
        //cases covered:
            //primary and other forward             stored_line_position after extension completion can be self.start, self.cursor, other.cursor
                // i d k|\n>  //primary
                // i d|k>\n   //other
                // i d k|\n>  //primary stationary
                // i d|k \n>  //other extended forward
            //primary forward, other backward       stored_line_position after extension completion can be self.start, self.cursor, other.cursor
                // i d k|\n>  //primary
                // i d<k|\n   //other
                // i d k|\n>  //primary stationary
                // i d|k \n>  //other extended forward          //note: selection direction changes here, due to how extension is implemented
        assert_eq!(Selection::with_stored_line_position(2, 4, 3), Selection::new(3, 4).merge_overlapping(&Selection::new(2, 4), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_self_non_extended_backward_and_other_extended_backward(){
        let text = Rope::from("idk\n");
        //cases covered:
            //primary and other backward            stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
                //<i|d k \n     //primary
                // i<d|k \n     //other
                //<i|d k \n     //primary stationary
                //<i d|k \n     //other extended backward
            //primary backward, other forward       stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
                //<i|d k \n     //primary
                // i|d>k \n     //other
                //<i|d k \n     //primary stationary
                //<i d|k \n     //other extended backward       //note: selection direction changes here, due to how extension is implemented
        assert_eq!(Selection::with_stored_line_position(2, 0, 0), Selection::new(1, 0).merge_overlapping(&Selection::new(2, 0), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_self_non_extended_forward_and_other_extended_backward(){
        let text = Rope::from("idk\n");
        //cases covered:
            //primary and other forward             stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
                //|i>d k \n     //primary
                // i|d>k \n     //other
                //|i>d k \n     //primary stationary
                //<i d|k \n     //other extended backward       //note: selection direction changes here, due to how extension is implemented
            //primary forward, other backward       stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
                //|i>d k \n     //primary
                // i<d|k \n     //other
                //|i>d k \n     //primary stationary
                //<i d|k \n     //other extended backward
        assert_eq!(Selection::with_stored_line_position(2, 0, 0), Selection::new(0, 1).merge_overlapping(&Selection::new(2, 0), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_self_non_extended_backward_and_other_extended_forward(){
        let text = Rope::from("idk\n");
        //cases covered:
            //primary and other backward            stored_line_position after extension completion can be self.start, self.cursor, other.cursor
                // i d k<\n|  //primary
                // i d<k|\n   //other
                // i d k<\n|  //primary stationary
                // i d|k \n>  //other extended forward          //note: selection direction changes here, due to how extension is implemented
            //primary backward, other forward       stored_line_position after extension completion can be self.start, self.cursor, other.cursor
                // i d k<\n|  //primary
                // i d|k>\n   //other
                // i d k<\n|  //primary stationary
                // i d|k \n>  //other extended forward
        assert_eq!(Selection::with_stored_line_position(2, 4, 3), Selection::new(4, 3).merge_overlapping(&Selection::new(2, 4), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_self_non_extended_and_other_extended_forward_bar_semantics(){
        let text = Rope::from("idk\n");
        //cases covered:
            // i d k \n|    //primary
            // i d k|\n     //other
            // i d k \n|    //primary stationary
            // i d k|\n>    //other extended forward
        assert_eq!(Selection::with_stored_line_position(3, 4, 0), Selection::new(4, 4).merge_overlapping(&Selection::new(3, 4), &text, CursorSemantics::Bar).unwrap());
    }
    #[test] fn with_self_non_extended_and_other_extended_backward_bar_semantics(){
        let text = Rope::from("idk\n");
        //cases covered:
            //|i d k \n     //primary
            // i|d k \n     //other
            //|i d k \n     //primary stationary
            //<i|d k \n     //other extended backward
        assert_eq!(Selection::with_stored_line_position(1, 0, 0), Selection::new(0, 0).merge_overlapping(&Selection::new(1, 0), &text, CursorSemantics::Bar).unwrap());
    }

    #[test] fn with_self_extended_forward_and_other_non_extended_forward(){
        let text = Rope::from("idk\n");
        //cases covered:
            //primary and other forward             stored_line_position after extension completion can be self.cursor, other.start, other.cursor
                // i d|k>\n   //primary
                // i d k|\n>  //other
                // i d|k \n>  //primary extended forward
                // i d k|\n>  //other stationary
            //primary backward, other forward       stored_line_position after extension completion can be self.cursor, other.start, other.cursor
                // i d<k|\n     //primary
                // i d k|\n>    //other
                // i d|k \n>    //primary extended forward      //note: selection direction changes here, due to how extension is implemented
                // i d k|\n>    //other stationary
        assert_eq!(Selection::with_stored_line_position(2, 4, 3), Selection::new(2, 4).merge_overlapping(&Selection::new(3, 4), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_self_extended_backward_and_other_non_extended_backward(){
        let text = Rope::from("idk\n");
        //cases covered:
            //primary and other backward            stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
                // i<d|k \n     //primary
                //<i|d k \n     //other
                //<i d|k \n     //primary extended backward
                //<i|d k \n     //other stationary
            //primary forward, other backward       stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
                // i|d>k \n     //primary
                //<i|d k \n     //other
                //<i d|k \n     //primary extended backward     //note: selection direction changes here, due to how extension is implemented
                //<i|d k \n     //other stationary
        assert_eq!(Selection::with_stored_line_position(2, 0, 0), Selection::new(2, 0).merge_overlapping(&Selection::new(1, 0), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_self_extended_forward_and_other_non_extended_backward(){
        let text = Rope::from("idk\n");
        //cases covered:
            //primary and other backward            stored_line_position after extension completion can be self.cursor, other.start, other.cursor
                // i d<k|\n   //primary
                // i d k<\n|  //other
                // i d|k \n>  //primary extended forward        //note: selection direction changes here, due to how extension is implemented
                // i d k<\n|  //other stationary
            //primary forward, other backward       stored_line_position after extension completion can be self.cursor, other.start, other.cursor
                // i d|k>\n   //primary
                // i d k<\n|  //other
                // i d|k \n>  //primary extended forward
                // i d k<\n|  //extended stationary
        assert_eq!(Selection::with_stored_line_position(2, 4, 3), Selection::new(2, 4).merge_overlapping(&Selection::new(4, 3), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_self_extended_backward_and_other_non_extended_forward(){
        let text = Rope::from("idk\n");
        //cases covered:
            //primary and other forward             stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
                // i|d>k \n     //primary
                //|i>d k \n     //other
                //<i d|k \n     //primary extended backward     //note: selection direction changes here, due to how extension is implemented
                //|i>d k \n     //other stationary
            //primary backward, other forward       stored_line_position after extension completion can be self.start, self.cursor, other.start, other.cursor
                // i<d|k \n     //primary
                //|i>d k \n     //other
                //<i d|k \n     //primary extended backward
                //|i>d k \n     //other stationary
        assert_eq!(Selection::with_stored_line_position(2, 0, 0), Selection::new(2, 0).merge_overlapping(&Selection::new(0, 1), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_self_extended_forward_and_other_non_extended_bar_semantics(){
        let text = Rope::from("idk\n");
        //cases covered:
            // i d k|\n     //primary
            // i d k \n|    //other
            // i d k|\n>    //primary extended forward
            // i d k \n|    //other stationary
        assert_eq!(Selection::with_stored_line_position(3, 4, 0), Selection::new(3, 4).merge_overlapping(&Selection::new(4, 4), &text, CursorSemantics::Bar).unwrap());
    }
    #[test] fn with_self_extended_backward_and_other_non_extended_bar_semantics(){
        let text = Rope::from("idk\n");
        //cases covered:
            // i|d k \n     //primary
            //|i d k \n     //other
            //<i|d k \n     //primary extended backward
            //|i d k \n     //other stationary
        assert_eq!(Selection::with_stored_line_position(1, 0, 0), Selection::new(1, 0).merge_overlapping(&Selection::new(0, 0), &text, CursorSemantics::Bar).unwrap());
    }



//extended multiselections          these cases result from multiselection movements where both selections extend, with one overlapping the other
    #[test] fn with_extended_multiselections_with_self_and_other_forward(){
        let text = Rope::from("idk\n");
        //cases covered:
            //primary and other forward             stored_line_position after extension completion can be self.cursor, other.end
                // i d|k>\n     //primary
                // i|d>k \n     //other
                // i d|k \n>    //primary extended forward
                // i|d k>\n     //other extended forward
            //primary and other backward            stored_line_position after extension completion can be self.cursor, other.end
                // i d<k|\n     //primary
                // i<d|k \n     //other
                // i d|k \n>    //primary extended forward      //note: selection direction changes here, due to how extension is implemented
                // i|d k>\n     //other extended forward        //note: selection direction changes here, due to how extension is implemented
            //primary forward, other backward       stored_line_position after extension completion can be self.cursor, other.end
                // i d|k>\n     //primary
                // i<d|k \n     //other
                // i d|k \n>    //primary extended forward
                // i|d k>\n     //other extended forward        //note: selection direction changes here, due to how extension is implemeted
            //primary backward, other forward       stored_line_position after extension completion can be self.cursor, other.end
                // i d<k|\n     //primary
                // i|d>k \n     //other
                // i d|k \n>    //primary extended forward      //note: selection direction changes here, due to how extension is implemented
                // i|d k>\n     //other extended forward
        assert_eq!(Selection::with_stored_line_position(1, 4, 3), Selection::new(2, 4).merge_overlapping(&Selection::new(1, 3), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_extended_multiselections_with_self_and_other_backward(){
        let text = Rope::from("idk\n");
        //cases covered:
            //primary and other forward             stored_line_position after extension completion can be self.start, self.cursor
                // i|d>k \n     //primary
                // i d|k>\n     //other
                //<i d|k \n     //primary extended backward     //note: selection direction changes here, due to how extension is implemented
                // i<d k|\n     //other extended backward       //note: selection direction changes here, due to how extension is implemented
            //primary and other backward            stored_line_position after extension completion can be self.start, self.cursor
                // i<d|k \n     //primary
                // i d<k|\n     //other
                //<i d|k \n     //primary extended backward
                // i<d k|\n     //other extended backward
            //primary forward, other backward       stored_line_position after extension completion can be self.start, self.cursor
                // i|d>k \n     //primary
                // i d<k|\n     //other
                //<i d|k \n     //primary extended backward     //note: selection direction changes here, due to how extension is implemented
                // i<d k|\n     //other extended backward
            //primary backward, other forward       stored_line_position after extension completion can be self.start, self.cursor
                // i<d|k \n     //primary
                // i d|k>\n     //other
                //<i d|k \n     //primary extended backward
                // i<d k|\n     //other extended backward       //note: selection direction changes here, due to how extension is implemented
        assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(2, 0).merge_overlapping(&Selection::new(3, 1), &text, CursorSemantics::Block).unwrap());
    }
    #[test] fn with_extended_multiselections_with_self_and_other_forward_bar_semantics(){
        let text = Rope::from("idk\n");
        //cases covered:
            // i|d>k \n     //primary
            // i d|k>\n     //other
            // i|d k>\n     //primary extended forward
            // i d|k \n>    //other extended forward
        assert_eq!(Selection::with_stored_line_position(1, 4, 0), Selection::new(1, 3).merge_overlapping(&Selection::new(2, 4), &text, CursorSemantics::Bar).unwrap());
    }
    #[test] fn with_extended_multiselections_with_self_and_other_backward_bar_semantics(){
        let text = Rope::from("idk\n");
        //cases covered:
            // i d<k|\n     //primary
            // i<d|k \n     //other
            // i<d k|\n     //primary extended backward
            //<i d|k \n     //other extended backward
        assert_eq!(Selection::with_stored_line_position(3, 0, 0), Selection::new(3, 1).merge_overlapping(&Selection::new(2, 0), &text, CursorSemantics::Bar).unwrap());
    }
