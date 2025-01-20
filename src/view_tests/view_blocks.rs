use ropey::Rope;
use crate::view::View;
use crate::range::Range;

#[test]
fn view_blocks(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 2, 2);
    //[i d]k
    //[s o]m e
    // s h i t
    //[i d]k \n[s o]m e \n s h i t \n
    assert_eq!(vec![Range::new(0, 2), Range::new(4, 6)], view.view_blocks(&text, false));
    
    let view = View::new(0, 1, 2, 2);
    // i d k
    //[s o]m e
    //[s h]i t
    // i d k \n[s o]m e \n[s h]i t \n
    assert_eq!(vec![Range::new(4, 6), Range::new(9, 11)], view.view_blocks(&text, false));
    
    let view = View::new(1, 0, 2, 2);
    // i[d k]
    // s[o m]e
    // s h i t
    // i[d k]\n s[o m]e \n s h i t
    assert_eq!(vec![Range::new(1, 3), Range::new(5, 7)], view.view_blocks(&text, false));
    
    let text = Rope::from("idk\nsomething\nelse");
    let view = View::new(5, 0, 2, 2);
    // i d k    [   ]
    // s o m e t[h i]n g
    // e l s e
    //[]i d k \n s o m e t[h i]n g \n e l s e
    assert_eq!(vec![Range::new(0, 0), Range::new(9, 11)], view.view_blocks(&text, false));
    
    // i d k
    // s o m e
    // s[h i t  ]
    // i[d k    ]
    // s[o m e t]h i n g
    // e l s e
    //                      1                     2                    3
    //0 1 2 3  4 5 6 7 8  9 0 1 2 3  4 5 6 7  8 9 0 1 2 3 4 5 6 7  8 9 0 1 2  3
    // i d k \n s o m e \n s[h i t]\n i[d k]\n s[o m e t]h i n g \n e l s e \n
    let text = Rope::from("idk\nsome\nshit\nidk\nsomething\nelse\n");
    let view = View::new(1, 2, 4, 3);
    assert_eq!(vec![Range::new(10, 13), Range::new(15, 17), Range::new(19, 23)], view.view_blocks(&text, false));
    
    let text = Rope::from("idk\n\nsomething\n");
    let view = View::new(2, 0, 1, 3);
    // i d[k]
    //    [ ]
    // s o[m]e t h i n g
    // i d[k]\n[]\n s o[m]e t h i n g
    assert_eq!(vec![Range::new(2, 3), Range::new(4, 4), Range::new(7, 8)], view.view_blocks(&text, false));
    
    let text = Rope::from("\n\nidk\n");
    let view = View::new(0, 0, 2, 3);
    //[   ]
    //[   ]
    //[i d]k
    // \n \n i d k \n
    assert_eq!(vec![Range::new(0, 0), Range::new(1, 1), Range::new(2, 4)], view.view_blocks(&text, false));
}
