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

/// Returns a new instance of [`Selection`] with the [`Selection`] extended to the right.
pub fn selection_impl(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> Result<Selection, SelectionError>{    //TODO: ensure this can't extend past doc text end
    selection.assert_invariants(text, semantics);
    if selection.range.start == text.len_chars()
    || selection.range.end == text.len_chars()
    || selection.cursor(text, semantics) == text.len_chars(){return Err(SelectionError::ResultsInSameState);}

    selection.move_horizontally(1, text, Movement::Extend, Direction::Forward, semantics)
}

#[cfg(test)]
mod tests{
    use crate::utilities::extend_selection_right;
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
        let result = extend_selection_right::document_impl(&mut doc, semantics);
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
        assert!(extend_selection_right::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    //use ropey::Rope;
    //use crate::range::Range;
    //use crate::selection::{Selection, CursorSemantics, Direction};

    //#[test] fn sanity_check(){
    //    let text = Rope::from("idk\nsome\nshit\n");
    //    assert_eq!(14, text.len_chars());
    //}

// bar
    #[test] fn normal_use_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 0), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 1)], 0
        );
    }
    #[test] fn normal_use_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(0, 2), Direction::Forward, 1)], 0
        );
    }

    #[test] fn extends_to_doc_text_end_bar_semantics(){
        test(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(13, 13), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(13, 14), Direction::Forward, 0)], 0
        );
    }
    #[test] fn extends_to_doc_text_end_block_semantics(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(12, 13), Direction::Forward)], 0, 
            vec![Selection::with_stored_line_position(Range::new(12, 14), Direction::Forward, 4)], 0
        );
    }

    //TODO:
    //#[test] fn with_previously_backward_extended_selection(){
    //    test(
    //        CursorSemantics::Bar, 
    //        "idk\nsome\nshit\n", 
    //        vec![Selection::new(Range::new(0, 14), Direction::Backward)], 0,
    //        vec![Selection::with_stored_line_position(Range::new(1, 14), Direction::Backward, 1)], 0
    //    );
    //}

    #[test] fn errors_if_cursor_at_doc_text_end_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(14, 14), Direction::Forward)], 0
        );
    }
    #[test] fn errors_if_cursor_at_doc_text_end_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(13, 14), Direction::Forward)], 0
        );
    }

    #[test] fn errors_if_already_extended_forward_at_doc_text_end_bar_semantics(){
        test_error(
            CursorSemantics::Bar, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 14), Direction::Forward)], 0
        );
    }
    #[test] fn errors_if_already_extended_forward_at_doc_text_end_block_semantics(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 14), Direction::Forward)], 0
        );
    }
}
