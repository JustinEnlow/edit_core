use ropey::Rope;
use crate::view::View;

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
