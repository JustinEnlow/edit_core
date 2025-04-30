use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, Direction, CursorSemantics, Movement},
};
use ropey::Rope;

//front end converts to zero-based
pub fn document_impl(document: &mut Document, line_number: usize, semantics: CursorSemantics) -> Result<(), DocumentError>{
    //assert!(line_number > 0);
    //let line_number = line_number.saturating_sub(1);    //convert to zero based //should this conversion really be happening on the back end?
    if line_number >= document.text.len_lines(){return Err(DocumentError::InvalidInput);}
    
    //if let Ok(()) = document.clear_non_primary_selections(){};
    if let Ok(()) = crate::utilities::clear_non_primary_selections::document_impl(document){};
    match selection_impl(document.selections.primary(), line_number, &document.text, Movement::Move, semantics){
        Ok(new_selection) => {*document.selections.primary_mut() = new_selection;}
        Err(_) => {return Err(DocumentError::InvalidInput);}    //should be same state error
    }
    Ok(())
}

/// Returns a new instance of [`Selection`] with the cursor set to specified 0-based line number.
fn selection_impl(selection: &Selection, line_number: usize, text: &Rope, movement: Movement, semantics: CursorSemantics) -> Result<Selection, SelectionError>{
    selection.assert_invariants(text, semantics);
    assert!(line_number < text.len_lines());

    if line_number == text.char_to_line(selection.cursor(text, semantics)){return Err(SelectionError::ResultsInSameState);}
    
    let current_line = text.char_to_line(selection.cursor(text, semantics));
    let (amount, direction) = if line_number < current_line{
        (current_line.saturating_sub(line_number), Direction::Backward)
    }else{
        (line_number.saturating_sub(current_line), Direction::Forward)
    };
    selection.move_vertically(amount, text, movement, direction, semantics)
}

#[cfg(test)]
mod tests{
    use crate::utilities::move_to_line_number;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics},
    };
    use ropey::Rope;

    //fn test(semantics: CursorSemantics, text: &str, line_number: usize, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    let result = move_to_line_number::document_impl(&mut doc, line_number, semantics);
    //    assert!(!result.is_err());
    //    let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
    //    assert_eq!(expected_selections, doc.selections);
    //    assert!(!doc.is_modified());
    //}
    //fn test_error(semantics: CursorSemantics, text: &str, line_number: usize, selections: Vec<Selection>, primary: usize){
    //    let text = Rope::from(text);
    //    let mut doc = Document::new(semantics)
    //        .with_text(text.clone())
    //        .with_selections(Selections::new(selections, primary, &text, semantics));
    //    assert!(move_to_line_number::document_impl(&mut doc, line_number, semantics).is_err());
    //    assert!(!doc.is_modified());
    //}
    fn test(semantics: CursorSemantics, text: &str, line_number: usize, tuple_selections: Vec<(usize, usize, Option<usize>)>, primary: usize, tuple_expected_selections: Vec<(usize, usize, Option<usize>)>, expected_primary: usize){
        let text = Rope::from(text);
        let mut vec_selections = Vec::new();
        for tuple in tuple_selections{
            vec_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let selections = Selections::new(vec_selections, primary, &text, semantics);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections);
        let result = move_to_line_number::document_impl(&mut doc, line_number, semantics);
        assert!(!result.is_err());
        let mut vec_expected_selections = Vec::new();
        for tuple in tuple_expected_selections{
            vec_expected_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let expected_selections = Selections::new(vec_expected_selections, expected_primary, &text, semantics);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, line_number: usize, tuple_selections: Vec<(usize, usize, Option<usize>)>, primary: usize){
        let text = Rope::from(text);
        let mut vec_selections = Vec::new();
        for tuple in tuple_selections{
            vec_selections.push(Selection::new_from_components(tuple.0, tuple.1, tuple.2, &text, semantics));
        }
        let selections = Selections::new(vec_selections, primary, &text, semantics);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections);
        assert!(move_to_line_number::document_impl(&mut doc, line_number, semantics).is_err());
        assert!(!doc.is_modified());
    }

    //TODO: restricts cursor to line end, when stored line position > line width

    #[test] fn moves_to_line_number_bar_semantics(){
        //test(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    2, 
        //    vec![
        //        Selection::new(Range::new(0, 0), Direction::Forward),
        //        Selection::new(Range::new(4, 4), Direction::Forward)
        //    ], 0, 
        //    vec![Selection::with_stored_line_position(Range::new(9, 9), Direction::Forward, 0)], 0
        //);
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            2, 
            vec![
                (0, 0, None),
                (4, 4, None)
            ], 0, 
            vec![
                (9, 9, Some(0))
            ], 0
        );
    }
    #[test] fn moves_to_line_number_block_semantics(){
        //test(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    2, 
        //    vec![
        //        Selection::new(Range::new(0, 1), Direction::Forward),
        //        Selection::new(Range::new(4, 5), Direction::Forward)
        //    ], 0, 
        //    vec![Selection::with_stored_line_position(Range::new(9, 10), Direction::Forward, 0)], 0
        //);
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            2, 
            vec![
                (0, 1, None),
                (4, 5, None)
            ], 0, 
            vec![
                (9, 10, Some(0))
            ], 0
        );
    }

    #[test] fn errors_if_already_at_line_number_bar_semantics(){
        //test_error(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    1, 
        //    vec![Selection::new(Range::new(4, 4), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            1, 
            vec![
                (4, 4, None)
            ], 0
        );
    }
    #[test] fn errors_if_already_at_line_number_block_semantics(){
        //test_error(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    1, 
        //    vec![Selection::new(Range::new(4, 5), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            1, 
            vec![
                (4, 5, None)
            ], 0
        );
    }
    
    #[test] fn errors_if_invalid_line_number_bar_semantics(){   //0 is valid, since backend line numbers are 0 based
        //test_error(
        //    CursorSemantics::Bar, 
        //    "idk\nsome\nshit\n", 
        //    500, 
        //    vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            500, 
            vec![
                (0, 0, None)
            ], 0
        );
    }
    #[test] fn errors_if_invalid_line_number_block_semantics(){
        //test_error(
        //    CursorSemantics::Block, 
        //    "idk\nsome\nshit\n", 
        //    500, 
        //    vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0
        //);
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            500, 
            vec![
                (0, 1, None)
            ], 0
        );
    }
}
