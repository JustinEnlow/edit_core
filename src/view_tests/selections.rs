use ropey::Rope;
use crate::view::View;
use crate::Position;
use crate::selection::Selection;
use crate::selection2d::Selection2d;
use crate::selections::Selections;

#[test]
fn selections(){
    // selections in view
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(1, 2), Selection::new(5, 6)], 0, &text);
    let view = View::new(0, 0, 3, 3);
    //[i|d>k]
    //[s|o>m]e
    //[s h i]t
    assert_eq!(
        //Some(vec![Selection2d::new(Position::new(1, 0), Position::new(2, 0)), Selection2d::new(Position::new(1, 1), Position::new(2, 1))]),
        Some(vec![Selection2d::new(Position::new(1, 0), Position::new(2, 0)), Selection2d::new(Position::new(1, 1), Position::new(2, 1)), Selection2d::new(Position::new(0, 2), Position::new(0, 2))]),
        view.selections(&selections, &text)
    );
    
    // no selection in view
    let text = Rope::from("idk\nsome\nshit\n");
    let selections = Selections::new(vec![Selection::new(7, 8)], 0, &text);
    let view = View::new(0, 0, 2, 2);
    //[i d]k
    //[s o]m|e>
    // s h i t
    assert_eq!(Some(vec![
            Selection2d::new(Position::new(0, 0), Position::new(0, 0)), 
            Selection2d::new(Position::new(0, 1), Position::new(0, 1))
        ]), 
        view.selections(&selections, &text)
    );
    
    // mix of in view and out
    let text = Rope::from("idk\nsome\nshit\nidk\nsomething\nelse\n");
    //                       1                     2                    3
    // 0 1 2 3  4 5 6 7 8  9 0 1 2 3  4 5 6 7  8 9 0 1 2 3 4 5 6 7  8 9 0 1 2  3
    // |i d>k \n s o m|e \n s h>i t \n i d|k \n s o m>e t h i n g \n|e l s>e \n     //selections
    //  i d k \n s o m e \n s[h i t]\n i[d k]\n s[o m e t]h i n g \n e l s e \n     //view_blocks
    let selections = Selections::new(vec![Selection::new(0, 2), Selection::new(7, 11), Selection::new(16, 21), Selection::new(28, 31)], 0, &text);
    //|i d>k
    // s o m|e
    // s[h>i t  ]
    // i[d|k    ]
    // s[o m>e t]h i n g
    //|e l s>e
    let view = View::new(1, 2, 4, 3);
    assert_eq!(Some(vec![
            Selection2d::new(Position::new(0, 0), Position::new(1, 0)),
            //Selection2d::new(Position::new(1, 1), Position::new(2, 1)),
            Selection2d::new(Position::new(1, 1), Position::new(3, 1)),
            Selection2d::new(Position::new(0, 2), Position::new(2, 2))
        ]), 
        view.selections(&selections, &text)
    );
    
    let text = Rope::from("idk\n\nsomething\n");
    let view = View::new(2, 0, 1, 3);
    let selections = Selections::new(vec![Selection::new(1, 3), Selection::new(5, 11)], 0, &text);
    // i|d[k]>
    //    [ ]
    //|s o[m]e t h>i n g
    assert_eq!(Some(vec![
            Selection2d::new(Position::new(0, 0), Position::new(1, 0)),
            Selection2d::new(Position::new(0, 1), Position::new(0, 1)),
            Selection2d::new(Position::new(0, 2), Position::new(1, 2))
        ]),
        view.selections(&selections, &text)
    );
    
    // TODO: test multiple selections per line
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 3, 3);
    let selections = Selections::new(vec![Selection::new(0, 1), Selection::new(2, 3), Selection::new(5, 6)], 0, &text);
    //[|i>d|k>]
    //[s|o>m]e
    //[s h i]t
    assert_eq!(Some(vec![
            Selection2d::new(Position::new(0, 0), Position::new(1, 0)),
            Selection2d::new(Position::new(2, 0), Position::new(3, 0)),
            Selection2d::new(Position::new(1, 1), Position::new(2, 1)),
            Selection2d::new(Position::new(0, 2), Position::new(0, 2)),
        ]),
        view.selections(&selections, &text)
    );
}
