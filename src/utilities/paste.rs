use crate::{
    document::{Document, DocumentError},
    selection::CursorSemantics
};

/// Insert clipboard contents at cursor position(s).
pub fn document_impl(document: &mut Document, use_hard_tab: bool, tab_width: usize, semantics: CursorSemantics) -> Result<(), DocumentError>{
    crate::utilities::insert_string::document_impl(document, &document.clipboard.clone(), use_hard_tab, tab_width, semantics)
}

#[cfg(test)]
mod tests{
    use crate::document::Document;
    use crate::range::Range;
    use crate::selection::{Selection, CursorSemantics, Direction};
    use crate::selections::Selections;
    use ropey::Rope;

    fn test(text: &str, selections: Vec<Selection>, primary: usize, clipboard: &str, expected_selections: Vec<Selection>, expected_primary: usize, expected_text: &str, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text, semantics))
            .with_clipboard(clipboard.to_string());
        let _ = crate::utilities::paste::document_impl(&mut doc, false, 4, semantics);
        let expected_text = Rope::from(expected_text);
        assert_eq!(expected_text, doc.text);
        let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
        assert_eq!(expected_selections, doc.selections);
    }
    fn test_error(text: &str, selections: Vec<Selection>, primary: usize, clipboard: &str, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text, semantics))
            .with_clipboard(clipboard.to_string());
        assert!(crate::utilities::paste::document_impl(&mut doc, false, 4, semantics).is_err());
    }

    #[test] fn paste_single_selection_block_semantics(){
        test(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(9, 10), Direction::Forward)], 0, 
            "other\n", 
            vec![Selection::with_stored_line_position(Range::new(15, 16), Direction::Forward, 0)], 0, 
            "idk\nsome\nother\nshit\n", 
            CursorSemantics::Block
        );
    }
    #[test] fn paste_single_selection_bar_semantics(){
        test(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(9, 9), Direction::Forward)], 0, 
            "other\n", 
            vec![Selection::with_stored_line_position(Range::new(15, 15), Direction::Forward, 0)], 0, 
            "idk\nsome\nother\nshit\n", 
            CursorSemantics::Bar
        );
    }
    //TODO: paste_multi_selection_block_semantics
    //TODO: paste_multi_selection_bar_semantics

    #[test] fn errors_if_empty_clipboard(){
        test_error(
            "idk\nshit\n", 
            vec![Selection::new(Range::new(4, 5), Direction::Forward)], 0, 
            "", 
            CursorSemantics::Block
        );
    }

}
