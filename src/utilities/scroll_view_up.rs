use crate::{
    document::{Document, DocumentError},
    view::{View, ViewError},
};

pub fn document_impl(document: &mut Document, amount: usize) -> Result<(), DocumentError>{
    match view_impl(&document.client_view, amount){
        Ok(view) => {document.client_view = view}
        Err(e) => {
            match e{
                ViewError::InvalidInput => {return Err(DocumentError::InvalidInput);}
                ViewError::ResultsInSameState => {return Err(DocumentError::InvalidInput);} //need error same state in document...
            }
        }
    }

    Ok(())
}

/// Returns a new instance of [`View`] with `vertical_start` decreased by specified amount.
/// # Errors
///     //if `amount` is 0.
///     //if function would return a `View` with the same state.
fn view_impl(view: &View, amount: usize) -> Result<View, ViewError>{
    if amount == 0{return Err(ViewError::InvalidInput);}
    if view.vertical_start == 0{return Err(ViewError::ResultsInSameState);}
    Ok(View::new(view.horizontal_start, view.vertical_start.saturating_sub(amount), view.width, view.height))
}

#[cfg(test)]
mod tests{
    use crate::utilities::scroll_view_up;
    use crate::{
        document::Document,
        selection::CursorSemantics,
        view::View,
    };
    use ropey::Rope;

    fn test(text: &str, view: View, amount: usize, expected_text: &str, expected_view: View, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_view(view);
        let _ = scroll_view_up::document_impl(&mut doc, amount);
        assert_eq!(expected_text.to_string(), doc.client_view.text(&text));
        assert_eq!(expected_view, doc.client_view);
    }
    fn test_error(text: &str, view: View, amount: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_view(view);
        assert!(scroll_view_up::document_impl(&mut doc, amount).is_err());
    }

    #[test] fn scroll_up(){
        test(
            "idk\nsome\nshit\n", 
            View::new(0, 2, 2, 2), 1, 
            "so\nsh\n", 
            View::new(0, 1, 2, 2), 
            CursorSemantics::Block
        );

        test(
            "idk\nsome\nshit\n", 
            View::new(0, 2, 2, 2), 1, 
            "so\nsh\n", 
            View::new(0, 1, 2, 2), 
            CursorSemantics::Bar
        );
    }
    //TODO: test when amount > space left to scroll.    //does this saturate at doc bounds currently?

    #[test] fn errors_if_already_scrolled_up_all_the_way(){
        test_error(
            "idk\nsome\nshit\n", 
            View::new(0, 0, 2, 2), 1, 
            CursorSemantics::Block
        );
        test_error(
            "idk\nsome\nshit\n", 
            View::new(0, 0, 2, 2), 1, 
            CursorSemantics::Bar
        );
    }

    #[test] fn errors_if_amount_is_zero(){
        test_error(
            "idk\nsome\nshit\n", 
            View::new(0, 1, 2, 2), 0, 
            CursorSemantics::Block
        );
        test_error(
            "idk\nsome\nshit\n", 
            View::new(0, 1, 2, 2), 0, 
            CursorSemantics::Bar
        );
    }
}
