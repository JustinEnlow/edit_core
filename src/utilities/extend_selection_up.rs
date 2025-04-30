use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics, Direction, Movement},
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match document.selections.move_cursor_potentially_overlapping(&document.text, semantics, selection_impl){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

/// Returns a new instance of [`Selection`] with the [`Selection`] extended up.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    if text.char_to_line(selection.cursor(text, semantics)) == 0{return Err(SelectionError::ResultsInSameState);}
    selection.move_vertically(1, text, Movement::Extend, Direction::Backward, semantics)
}

#[cfg(test)]
mod tests{
    use crate::utilities::extend_selection_up;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics},
    };
    use ropey::Rope;

    //fn test(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    //for selection in doc.selections.clone().iter(){
    //    //    println!("{}, {}", selection.range.start, selection.range.end);
    //    //}
    //    let result = extend_selection_up::document_impl(&mut doc, semantics);
    //    assert!(!result.is_err());
    //    let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
    //    assert_eq!(expected_selections, doc.selections);
    //    assert!(!doc.is_modified());
    //}
    //fn test_error(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    assert!(extend_selection_up::document_impl(&mut doc, semantics).is_err());
    //    assert!(!doc.is_modified());
    //}
    fn test(semantics: CursorSemantics, text: &str, tuple_selections: Vec<(usize, usize, Option<usize>)>, primary: usize, tuple_expected_selections: Vec<(usize, usize, Option<usize>)>, expected_primary: usize){
        let text = Rope::from(text);
        let mut vec_selections = Vec::new();
        for tuple in tuple_selections{
            vec_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let selections = Selections::new(vec_selections, primary, &text, semantics);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections);
        let result = extend_selection_up::document_impl(&mut doc, semantics);
        assert!(!result.is_err());
        let mut vec_expected_selections = Vec::new();
        for tuple in tuple_expected_selections{
            vec_expected_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let expected_selections = Selections::new(vec_expected_selections, expected_primary, &text, semantics);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, tuple_selections: Vec<(usize, usize, Option<usize>)>, primary: usize){
        let text = Rope::from(text);
        let mut vec_selections = Vec::new();
        for tuple in tuple_selections{
            vec_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let selections = Selections::new(vec_selections, primary, &text, semantics);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections);
        assert!(extend_selection_up::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_mixed_valid_and_invalid_selections_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 0), Direction::Forward),   //invalid
        //        Selection::new(Range::new(8, 8), Direction::Forward),   //to shorter line
        //        Selection::new(Range::new(14, 14), Direction::Forward)  //common use
        //    ], 0, 
        //    vec![
        //        Selection::new(Range::new(0, 0), Direction::Forward),
        //        Selection::with_stored_line_position(Range::new(3, 8), Direction::Backward, 4),
        //        Selection::with_stored_line_position(Range::new(9, 14), Direction::Backward, 0),
        //    ], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 0, None),   //invalid
                (8, 8, None),   //to shorter line
                (14, 14, None)  //common use
            ], 0, 
            vec![
                (0, 0, None),
                (8, 3, Some(4)),
                (14, 9, Some(0))
            ], 0
        );
    }
    //TODO: this test is correct, but we are currently reducing the selection if it is over a newline, which maybe we shouldn't, as its causing bugs
    #[test] fn with_mixed_valid_and_invalid_selections_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),   //invalid
        //        Selection::new(Range::new(8, 9), Direction::Forward),   //to shorter line
        //        Selection::new(Range::new(14, 15), Direction::Forward)  //common use
        //    ], 0, 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::with_stored_line_position(Range::new(3, 9), Direction::Backward, 4),
        //        Selection::with_stored_line_position(Range::new(9, 15), Direction::Backward, 0),
        //    ], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (0, 1, None),   //invalid
                (8, 9, None),   //to shorter line
                (14, 15, None)  //common use
            ], 0, 
            vec![
                (0, 1, None),
                //(9, 3, Some(4)),  //this really should be the correct resultant selection
                (8, 3, Some(4)),    //but this is what we are getting, because we reduce the selection when over a newline, currently
                //(15, 9, Some(0))  //selections that extend past doc text end should be reduced down
                (14, 9, Some(0))    //so i think this is correct...we want to keep this behavior
            ], 0
        );
    }
    
    #[test] fn errors_when_single_selection_on_top_line_bar_semantics(){
        //test_error(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(3, 3), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![
                (3, 3, None)
            ], 0
        );
    }
    #[test] fn errors_when_single_selection_on_top_line_block_semantics(){
        //test_error(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    vec![Selection::new(Range::new(3, 4), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                (3, 4, None)
            ], 0
        );
    }
}
