use crate::{
    document::{Document, DocumentError},
    selection::{Selection, SelectionError, CursorSemantics, Movement},
    text_util,
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match document.selections.move_cursor_potentially_overlapping(&document.text, semantics, selection_impl){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e))}   //though, should only return SelectionsError::ResultsInSameState
    }
    Ok(())
}

//TODO: this seems to be misbehaving when selection already extend left word boundary, and then extend right word boundary triggered.
//only when cursor over character that can be a beginning or ending word boundary...
/// Returns a new instance of [`Selection`] with cursor extended right to the nearest word boundary.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{  //TODO: ensure this can't extend past doc text end
    selection.assert_invariants(text, semantics);
    if selection.range.start == text.len_chars()
    || selection.range.end == text.len_chars()
    || selection.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}
        
    let goal_index = text_util::next_word_boundary(selection.head(), text);
    match semantics{
        CursorSemantics::Bar => {
            selection.put_cursor(goal_index, text, Movement::Extend, semantics, true)
        }
        CursorSemantics::Block => {
            if goal_index == text.len_chars(){
                //self.put_cursor(goal_index, text, Movement::Extend, semantics, true)
                selection.put_cursor(text_util::previous_grapheme_index(text.len_chars(), text), text, Movement::Extend, semantics, true)
            }else{
                selection.put_cursor(
                    text_util::previous_grapheme_index(goal_index, text), 
                    text, 
                    Movement::Extend, 
                    semantics, 
                    true
                )
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::utilities::extend_selection_word_boundary_forward;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range,
    };
    use ropey::Rope;

    fn test(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let result = extend_selection_word_boundary_forward::document_impl(&mut doc, semantics);
        assert!(!result.is_err());
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(extend_selection_word_boundary_forward::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    //#[test] fn sanity_check(){
    //    let text = Rope::from("idk\nsome\nshit\n");
    //    assert_eq!(14, text.len_chars());
    //}
    //
    //#[test] fn extend_right_word_boundary(){
    //    test(
    //        CursorSemantics::Block, 
    //        "use std::error::Error;", 
    //        vec![
    //            Selection::new(Range::new(0, 1), Direction::Forward)
    //        ], 0, 
    //        vec![
    //            Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 2)
    //        ], 0
    //    );
    //    test(
    //        CursorSemantics::Bar, 
    //        "use std::error::Error;", 
    //        vec![
    //            Selection::new(Range::new(0, 0), Direction::Forward)
    //        ], 0, 
    //        vec![
    //            Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 3)
    //        ], 0
    //    );
    //}

    #[test] fn with_multiple_valid_selections(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 2),
                Selection::with_stored_line_position(Range::new(4, 8), Direction::Forward, 3)
            ], 0
        );
    }
    #[test] fn with_mixed_valid_and_invalid_selections(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(13, 14), Direction::Forward)
            ], 0, 
            vec![
                Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 2),
                Selection::new(Range::new(13, 14), Direction::Forward)
            ], 0
        );
    }
    //should error if single selection at doc end
    //TODO: test with previously forward extended, with cursor over non word char

    #[test] fn normal_use_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 3)], 0
        );
    }
    #[test] fn normal_use_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(0, 3), Direction::Forward, 2)], 0
        );
    }
    
    #[test] fn extends_to_doc_end_from_doc_text_end_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(13, 13), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(13, 14), Direction::Forward, 0)], 0
        );
    }
    #[test] fn extends_to_doc_end_from_doc_text_end_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(12, 13), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(12, 14), Direction::Forward, 4)], 0
        );
    }

    #[test] fn errors_if_cursor_at_doc_end_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(14, 14), Direction::Forward)], 0
        );
    }
    #[test] fn errors_if_cursor_at_doc_end_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(13, 14), Direction::Forward)], 0
        );
    }

    #[test] fn errors_if_already_extended_forward_to_doc_end_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 14), Direction::Forward)], 0
        );
    }
    #[test] fn errors_if_already_extended_forward_to_doc_end_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 14), Direction::Forward)], 0
        );
    }

    //TODO: actually, this should work... it should move the cursor from 0 to 3...
    #[test] fn errors_if_already_extended_backward_from_doc_end_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 14), Direction::Backward)], 0
        );
    }
    #[test] fn errors_if_already_extended_backward_from_doc_end_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 14), Direction::Backward)], 0
        );
    }
}
