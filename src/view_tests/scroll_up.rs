//use ropey::Rope;
//use crate::view::View;
//
//#[test]
//fn scroll_up(){
//    let text = Rope::from("idk\nsome\nshit\n");
//    
//    // scrolls when vertical space remaining in text
//    let view = View::new(0, 2, 2, 2);
//    assert_eq!(View::new(0, 1, 2, 2), view.scroll_up(1).unwrap());
//    assert_eq!(String::from("so\nsh\n"), view.scroll_up(1).unwrap().text(&text));
//    
//    //errors if already scrolled all the way up
//    let view = View::new(0, 0, 2, 2);
//    assert!(view.scroll_up(1).is_err());
//
//    //errors if amount is 0
//    assert!(view.scroll_up(0).is_err());
//}
//