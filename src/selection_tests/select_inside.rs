use ropey::Rope;
use crate::selection::Selection;

#[test] fn idk(){
                               //0123456789012
    let text = Rope::from("  something  ");
    let selection = Selection::new(5, 6);
    println!("{:?}", text.slice(0..=selection.range.start).chars().reversed());
    //assert_eq!(Selection::new(1, 10), selection.select_inside_instances_of_single_char(' ', &text).unwrap());
    assert_eq!(Selection::new(2, 11), selection.select_inside_instances_of_single_char(' ', &text).unwrap());
}

#[test] fn select_inside_pair(){
                               //01234
    let text = Rope::from("(idk)");
    let selection = Selection::new(2, 3);
    assert_eq!(Selection::new(1, 4), selection.select_inside_pair('(', ')', &text).unwrap());
}
