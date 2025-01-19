use ropey::Rope;
use crate::view::View;

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
