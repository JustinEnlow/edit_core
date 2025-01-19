use ropey::Rope;
use crate::view::View;

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
