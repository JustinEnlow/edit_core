use ropey::Rope;
use edit_core::view::View;
use edit_core::selection::{Selection, CursorSemantics};
use edit_core::selection2d::Selection2d;
use edit_core::selections::Selections;
use edit_core::Position;

#[test]
fn scroll_down(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // scrolls when vertical space remaining in text
    let view = View::new(0, 0, 2, 2);
    assert_eq!(View::new(0, 1, 2, 2), view.scroll_down(1, &text).unwrap());
    assert_eq!(String::from("so\nsh\n"), view.scroll_down(1, &text).unwrap().text(&text));
    
    //errors if already scrolled all the way down
    let view = View::new(0, 2, 2, 2);
    assert!(view.scroll_down(1, &text).is_err());

    //errors if amount is 0
    assert!(view.scroll_down(0, &text).is_err());
}

#[test]
fn scroll_left(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // scrolls when horizontal space remaining in text
    let view = View::new(2, 0, 2, 2);
    assert_eq!(View::new(1, 0, 2, 2), view.scroll_left(1).unwrap());
    assert_eq!(String::from("dk\nom\n"), view.scroll_left(1).unwrap().text(&text));

    //errors if already scrolled all the way left
    let view = View::new(0, 0, 2, 2);
    assert!(view.scroll_left(1).is_err());

    //errors if amount is 0
    assert!(view.scroll_left(0).is_err());
}

#[test]
fn scroll_right(){
    let text = Rope::from("idk\nsome\nshit\n");

    // scrolls when horizontal space remaining in text
    let view = View::new(0, 0, 2, 2);
    assert_eq!(View::new(1, 0, 2, 2), view.scroll_right(1, &text).unwrap());
    assert_eq!(String::from("dk\nom\n"), view.scroll_right(1, &text).unwrap().text(&text));
    
    //errors if already scrolled all the way right
    let view = View::new(2, 0, 2, 2);
    assert!(view.scroll_right(1, &text).is_err());

    //errors if amount is 0
    assert!(view.scroll_right(0, &text).is_err());
    
    
}

#[test]
fn scroll_up(){
    let text = Rope::from("idk\nsome\nshit\n");
    
    // scrolls when vertical space remaining in text
    let view = View::new(0, 2, 2, 2);
    assert_eq!(View::new(0, 1, 2, 2), view.scroll_up(1).unwrap());
    assert_eq!(String::from("so\nsh\n"), view.scroll_up(1).unwrap().text(&text));
    
    //errors if already scrolled all the way up
    let view = View::new(0, 0, 2, 2);
    assert!(view.scroll_up(1).is_err());

    //errors if amount is 0
    assert!(view.scroll_up(0).is_err());
}

#[test]
fn should_scroll(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 2, 2);
    
    // in view
    let selection = Selection::new(0, 0);
    assert_eq!(false, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    let selection = Selection::new(0, 1);
    assert_eq!(false, view.should_scroll(&selection, &text, CursorSemantics::Block));
    
    // out of view horizontally
    let selection = Selection::new(3, 3);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    let selection = Selection::new(3, 4);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Block));
    
    // out of view vertically
    let selection = Selection::new(10, 10);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Bar));
    let selection = Selection::new(10, 11);
    assert_eq!(true, view.should_scroll(&selection, &text, CursorSemantics::Block));
}

#[test]
fn scroll_following_cursor(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 2, 2);
    
    // return self when primary [`Selection`] `head` within [`View`] bounds
    let selection = Selection::new(0, 0);
    assert_eq!(view, view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar));
    assert_eq!(String::from("id\nso\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar).text(&text));
    let selection = Selection::new(0, 1);
    assert_eq!(view, view.scroll_following_cursor(&selection, &text, CursorSemantics::Block));
    assert_eq!(String::from("id\nso\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Block).text(&text));
    
    // returns proper [`View`] when [`Selection`] `head` outside [`View`] bounds
    let selection = Selection::new(13, 13);
    assert_eq!(View::new(3, 1, 2, 2), view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar));
    assert_eq!(String::from("e\nt\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Bar).text(&text));
    let selection = Selection::new(13, 14);
    assert_eq!(View::new(3, 1, 2, 2), view.scroll_following_cursor(&selection, &text, CursorSemantics::Block));
    assert_eq!(String::from("e\nt\n"), view.scroll_following_cursor(&selection, &text, CursorSemantics::Block).text(&text));
}

// TODO: make separate tests for a view with odd number of lines, and a view with even number of lines. ensure behavior is as expected
#[test]
fn center_vertically_around_cursor(){
    let text = Rope::from("idk\nsome\nshit\nand\nsomething\nelse\n");   //len 33
    //let view = View::new(0, 0, 1, 1);
    
    //works when past half the view in text
    //let selection = Selection::new(14, 14);
    //assert_eq!(View::new(0, 3, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).unwrap());
    //let selection = Selection::new(14, 15);
    //assert_eq!(View::new(0, 3, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).unwrap());
    
    // old tests
        //let selection = Selection::new(0, 0);
        //assert_eq!(view, view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar));
        //let selection = Selection::new(0, 1);
        //assert_eq!(view, view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block));
        //
        //let selection = Selection::new(33, 33);
        //assert_eq!(View::new(0, 6, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar));
        //let selection = Selection::new(33, 34);
        //assert_eq!(View::new(0, 6, 1, 1), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block));

    //errors near doc start
    //let selection = Selection::new(0, 0);
    //assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).is_err());
    //let selection = Selection::new(0, 1);
    //assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).is_err());

    //errors near doc end
    //let selection = Selection::new(33, 33);
    //assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).is_err());
    //let selection = Selection::new(33, 34);
    //assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).is_err());

    let view = View::new(0, 0, 3, 3);
    //|i d k|
    //|s o m|e
    //|s h i|t
    // a n d
    // s o m e t h i n g
    // e l s e

    // works when cursor past half view height and before doc end - half view height
    let selection = Selection::new(9, 9);
    assert_eq!(View::new(0, 1, 3, 3), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).unwrap());
    let selection = Selection::new(9, 10);
    assert_eq!(View::new(0, 1, 3, 3), view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).unwrap());

    // errors when cursor before half view height
    let selection = Selection::new(0, 0);
    assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).is_err());
    let selection = Selection::new(0, 1);
    assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).is_err());

    // errors when cursor after doc end - half view height
    let selection = Selection::new(33, 33);
    assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).is_err());
    let selection = Selection::new(33, 34);
    assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Block).is_err());

    // errors when cursor already centered
    let selection = Selection::new(4, 4);
    assert!(view.center_vertically_around_cursor(&selection, &text, CursorSemantics::Bar).is_err());
}

#[test]
fn text(){
    // empty text
    let text = Rope::from("");
    let view = View::new(0, 0, 5, 5);
    assert_eq!(String::from("\n"), view.text(&text));   //is this correct? should this just be an empty line?   //after testing empty file, it seems fine.

    // exact fit
    let text = Rope::from("Line1\nLine2\nLine3\nLine4\nLine5\n");
    let view = View::new(0, 0, 5, 5);
    assert_eq!(String::from("Line1\nLine2\nLine3\nLine4\nLine5\n"), view.text(&text));

    // view bigger than text
    let text = Rope::from("Line1\nLine2\nLine3\nLine4\nLine5\n");
    let view = View::new(0, 0, 6, 6);
    assert_eq!(String::from("Line1\nLine2\nLine3\nLine4\nLine5\n\n"), view.text(&text));

    // vertical clip
    let text = Rope::from("Line1\nLine2\nLine3\nLine4\nLine5\n");
    let view = View::new(0, 0, 5, 2);
    assert_eq!(String::from("Line1\nLine2\n"), view.text(&text));

    // horizontal clip
    let text = Rope::from("Line1\nLine2\nLine3\nLine4\nLine5\n");
    let view = View::new(0, 0, 2, 5);
    assert_eq!(String::from("Li\nLi\nLi\nLi\nLi\n"), view.text(&text));

    // with vertical start
    let text = Rope::from("Line1\nLine2\nLine3\nLine4\nLine5\n");
    let view = View::new(0, 2, 2, 2);
    assert_eq!(String::from("Li\nLi\n"), view.text(&text));
    
    // with horizontal start
    let text = Rope::from("Line1\nLine2\nLine3\nLine4\nLine5\n");
    let view = View::new(2, 0, 2, 2);
    assert_eq!(String::from("ne\nne\n"), view.text(&text));

    // with space before line text start
}

#[test]
fn line_numbers(){
    // empty text
    let text = Rope::from("");
    let view = View::new(0, 0, 5, 5);
    assert_eq!(String::from("1"), view.line_numbers(&text));

    // normal
    let text = Rope::from("Line1\nLine2\nLine3\nLine4\nLine5\n");
    let view = View::new(0, 0, 5, 5);
    assert_eq!(String::from("1\n2\n3\n4\n5"), view.line_numbers(&text));

    // with text < view
    let text = Rope::from("Line1\nLine2\nLine3\nLine4\nLine5\n");
    let view = View::new(0, 0, 10, 10);
    assert_eq!(String::from("1\n2\n3\n4\n5\n6"), view.line_numbers(&text)); //counts the extra line after last newline

    // with vertical start
    let text = Rope::from("Line1\nLine2\nLine3\nLine4\nLine5\n");
    let view = View::new(0, 2, 5, 5);
    assert_eq!(String::from("3\n4\n5\n6"), view.line_numbers(&text));

    // with horizontal start. should be no different than normal
    let text = Rope::from("Line1\nLine2\nLine3\nLine4\nLine5\n");
    let view = View::new(2, 0, 5, 5);
    assert_eq!(String::from("1\n2\n3\n4\n5"), view.line_numbers(&text));
}

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

#[test]
fn view_blocks(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 2, 2);
    //[i d]k
    //[s o]m e
    // s h i t
    //[i d]k \n[s o]m e \n s h i t \n
    assert_eq!(vec![Selection::new(0, 2), Selection::new(4, 6)], view.view_blocks(&text, false));
    
    let view = View::new(0, 1, 2, 2);
    // i d k
    //[s o]m e
    //[s h]i t
    // i d k \n[s o]m e \n[s h]i t \n
    assert_eq!(vec![Selection::new(4, 6), Selection::new(9, 11)], view.view_blocks(&text, false));
    
    let view = View::new(1, 0, 2, 2);
    // i[d k]
    // s[o m]e
    // s h i t
    // i[d k]\n s[o m]e \n s h i t
    assert_eq!(vec![Selection::new(1, 3), Selection::new(5, 7)], view.view_blocks(&text, false));
    
    let text = Rope::from("idk\nsomething\nelse");
    let view = View::new(5, 0, 2, 2);
    // i d k    [   ]
    // s o m e t[h i]n g
    // e l s e
    //[]i d k \n s o m e t[h i]n g \n e l s e
    assert_eq!(vec![Selection::new(0, 0), Selection::new(9, 11)], view.view_blocks(&text, false));
    
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
    assert_eq!(vec![Selection::new(10, 13), Selection::new(15, 17), Selection::new(19, 23)], view.view_blocks(&text, false));
    
    let text = Rope::from("idk\n\nsomething\n");
    let view = View::new(2, 0, 1, 3);
    // i d[k]
    //    [ ]
    // s o[m]e t h i n g
    // i d[k]\n[]\n s o[m]e t h i n g
    assert_eq!(vec![Selection::new(2, 3), Selection::new(4, 4), Selection::new(7, 8)], view.view_blocks(&text, false));
    
    let text = Rope::from("\n\nidk\n");
    let view = View::new(0, 0, 2, 3);
    //[   ]
    //[   ]
    //[i d]k
    // \n \n i d k \n
    assert_eq!(vec![Selection::new(0, 0), Selection::new(1, 1), Selection::new(2, 4)], view.view_blocks(&text, false));
}

#[test]
fn primary_cursor_position(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 5, 5);
    let selections = Selections::new(vec![Selection::new(0, 3)], 0, &text);
    assert_eq!(Some(Position::new(3, 0)), view.primary_cursor_position(&text, &selections, CursorSemantics::Bar));

    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 5, 5);
    let selections = Selections::new(vec![Selection::new(0, 3)], 0, &text);
    assert_eq!(Some(Position::new(2, 0)), view.primary_cursor_position(&text, &selections, CursorSemantics::Block));
}
#[test]
fn primary_cursor_position_with_cursor_outside_view(){
    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 5, 1);
    let selections = Selections::new(vec![Selection::new(9, 13)], 0, &text);
    assert_eq!(None, view.primary_cursor_position(&text, &selections, CursorSemantics::Bar));

    let text = Rope::from("idk\nsome\nshit\n");
    let view = View::new(0, 0, 5, 1);
    let selections = Selections::new(vec![Selection::new(9, 13)], 0, &text);
    assert_eq!(None, view.primary_cursor_position(&text, &selections, CursorSemantics::Block));
}
