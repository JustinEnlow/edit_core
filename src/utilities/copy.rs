use crate::{
    document::{Document, DocumentError},
    selections::SelectionsError
};

/// Copy single selection to clipboard.
/// Ensure single selection when calling this function.
pub fn document_impl(document: &mut Document) -> Result<(), DocumentError>{
    if document.selections.count() > 1{return Err(DocumentError::SelectionsError(SelectionsError::MultipleSelections));}
    
    let selection = document.selections.primary().clone();
    // Copy the selected text to the clipboard
    document.clipboard = document.text.slice(selection.range.start..selection.range.end).to_string();

    Ok(())
}

#[cfg(test)]
mod tests{
    use crate::utilities::copy;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range,
    };
    use ropey::Rope;

    fn test(text: &str, selections: Vec<Selection>, primary: usize, expected_clipboard: &str, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let _ = copy::document_impl(&mut doc);
        assert_eq!(expected_clipboard.to_string(), doc.clipboard);
    }
    fn test_error(text: &str, selections: Vec<Selection>, primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(copy::document_impl(&mut doc).is_err());
    }

    //TODO: copy with no selection extension
        //should fail with bar semantics?...
        //should copy single char with block semantics

    #[test] fn copy_with_selection_direction_forward_block_semantics(){
        test(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(4, 9), Direction::Forward)], 0, 
            "some\n", 
            CursorSemantics::Block
        );
    }
    #[test] fn copy_with_selection_direction_forward_bar_semantics(){
        test(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(4, 9), Direction::Forward)], 0, 
            "some\n", 
            CursorSemantics::Bar
        );
    }

    #[test] fn copy_with_selection_direction_backward_block_semantics(){
        test(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(4, 9), Direction::Backward)], 0, 
            "some\n", 
            CursorSemantics::Block
        );
    }
    #[test] fn copy_with_selection_direction_backward_bar_semantics(){
        test(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(4, 9), Direction::Backward)], 0, 
            "some\n", 
            CursorSemantics::Bar
        );
    }

    #[test] fn copy_with_multiple_selections_should_error(){
        test_error(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward), 
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            CursorSemantics::Block
        );
        test_error(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward), 
                Selection::new(Range::new(4, 4), Direction::Forward)
            ], 0, 
            CursorSemantics::Bar
        );
    }
}
