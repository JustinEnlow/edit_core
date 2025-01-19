use ropey::Rope;
use crate::view::View;

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
