use crate::{
    document::{Document, DocumentError},
    selections::SelectionsError,
    selection::CursorSemantics,
};

/// Cut single selection.
/// Copies text to clipboard and removes selected text from document.
/// Ensure single selection when calling this function.
pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    if document.selections.count() > 1{return Err(DocumentError::SelectionsError(SelectionsError::MultipleSelections));}

    let selection = document.selections.primary_mut();
    // Copy the selected text to the clipboard
    document.clipboard = document.text.slice(selection.range.start..selection.range.end).to_string();
    crate::utilities::delete::document_impl(document, semantics)   //notice this is returning the result from delete
}

#[cfg(test)]
mod tests{
    use crate::utilities::cut;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range
    };
    use ropey::Rope;

    fn test(text: &str, selections: Vec<Selection>, primary: usize, expected_text: &str, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics, expected_clipboard: &str){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let _ = cut::document_impl(&mut doc, semantics);
        let expected_text = Rope::from(expected_text);
        assert_eq!(expected_text, doc.text);
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert_eq!(expected_clipboard.to_string(), doc.clipboard);
    }
    fn test_error(text: &str, selections: Vec<Selection>, primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(cut::document_impl(&mut doc, semantics).is_err())
    }

    #[test] fn cut_with_selection_direction_forward_block_semantics(){
        test(
            "idk\nsome\nshit\n",
            vec![Selection::new(Range::new(4, 9), Direction::Forward)], 0, 
            "idk\nshit\n", 
            vec![Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 0)], 0, 
            CursorSemantics::Bar,
            "some\n"
        );        
    }
    #[test] fn cut_with_selection_direction_forward_bar_semantics(){
        test(
            "idk\nsome\nshit\n",
            vec![Selection::new(Range::new(4, 9), Direction::Forward)], 0, 
            "idk\nshit\n", 
            vec![Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0)], 0, 
            CursorSemantics::Block,
            "some\n"
        );
    }

    #[test] fn cut_with_selection_direction_backward_block_semantics(){
        test(
            "idk\nsome\nshit\n",
            vec![Selection::new(Range::new(4, 9), Direction::Backward)], 0, 
            "idk\nshit\n", 
            vec![Selection::with_stored_line_position(Range::new(4, 5), Direction::Forward, 0)], 0, 
            CursorSemantics::Block,
            "some\n"
        );
    }
    #[test] fn cut_with_selection_direction_backward_bar_semantics(){
        test(
            "idk\nsome\nshit\n",
            vec![Selection::new(Range::new(4, 9), Direction::Backward)], 0, 
            "idk\nshit\n", 
            vec![Selection::with_stored_line_position(Range::new(4, 4), Direction::Forward, 0)], 0, 
            CursorSemantics::Bar,
            "some\n"
        );
    }

    #[test] fn cut_with_multiple_selections_returns_error(){
        test_error(
            "idk\nsome\nshit\n",
            vec![
                Selection::new(Range::new(0, 3), Direction::Forward),
                Selection::new(Range::new(4, 7), Direction::Forward)
            ], 0, 
            CursorSemantics::Bar
        );
        test_error(
            "idk\nsome\nshit\n",
            vec![
                Selection::new(Range::new(0, 3), Direction::Forward),
                Selection::new(Range::new(4, 7), Direction::Forward)
            ], 0, 
            CursorSemantics::Block
        );
    }
}
