use ropey::Rope;
use crate::selection::Selection;

//#[test] fn initial_test(){
//    let text = Rope::from("idk: Idk, some: Some, shit: Shit");  //len 32
//    let selection = Selection::new(0, text.len_chars());
//    assert_eq!(
//        Ok(
//            vec![
//                Selection::new(0, 8),
//                Selection::new(10, 20),
//                Selection::new(22, 32)
//            ]
//        ),
//        selection.split(", ", &text)
//    );
//}

#[test] fn initial_test(){
    let text = Rope::from("idk: Idk, some: Some, shit: Shit");  //len 32
    let selection = Selection::new(0, text.len_chars());
    assert_eq!(
        vec![
            Selection::new(0, 8),
            Selection::new(10, 20),
            Selection::new(22, 32)
        ],
        selection.split(", ", &text)
    );
}

#[test] fn follow_up_test(){
    let text = Rope::from("fn idk(idk: Idk, shit: Shit){"); //len 29
    let selection = Selection::new(7, 27);
    assert_eq!(
        vec![
            Selection::new(7, 15),
            Selection::new(17, 27)
        ],
        selection.split(", ", &text)
    );
}

#[test] fn with_split_at_start_of_selection(){
    let text = Rope::from(",idk,some,shit");
    let selection = Selection::new(0, text.len_chars());
    assert_eq!(
        vec![
            Selection::new(1, 4),
            Selection::new(5, 9),
            Selection::new(10, 14)
        ],
        selection.split(",", &text)
    );
}

#[test] fn returns_empty_vec_when_no_matching_pattern(){
    let text = Rope::from("idk\nsome\nshit\n");
    let selection = Selection::new(0, text.len_chars());
    assert_eq!(Vec::<Selection>::new(), selection.split("x", &text));
}
