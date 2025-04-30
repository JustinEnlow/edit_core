use crate::{
    document::{Document, DocumentError},
    view::{View, ViewError}
};
use ropey::Rope;

pub fn document_impl(document: &mut Document, amount: usize) -> Result<(), DocumentError>{
    match view_impl(&document.client_view, amount, &document.text){
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

/// Returns a new instance of [`View`] with `vertical_start` increased by specified amount.
/// # Errors
///     //if `amount` is 0.
///     //if function would return a `View` with the same state.
/// # Panics
///     //if `text` is invalid.
fn view_impl(view: &View, amount: usize, text: &Rope) -> Result<View, ViewError>{
    assert!(text.len_lines() > 0);

    if amount == 0{return Err(ViewError::InvalidInput);}

    let max_scrollable_position = text.len_lines().saturating_sub(view.height);
    if view.vertical_start == max_scrollable_position{return Err(ViewError::ResultsInSameState);}
    
    let new_vertical_start = view.vertical_start.saturating_add(amount);

    if new_vertical_start <= max_scrollable_position{
        Ok(View::new(view.horizontal_start, new_vertical_start, view.width, view.height))
    }else{
        Ok(View::new(view.horizontal_start, max_scrollable_position, view.width, view.height))
    }
}

#[cfg(test)]
mod tests{
    use crate::utilities::scroll_view_down;
    use crate::{
        document::Document,
        selection::CursorSemantics,
        view::View,
    };
    use ropey::Rope;

    fn test(semantics: CursorSemantics, text: &str, view: View, amount: usize, expected_text: &str, expected_view: View){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_view(view);
        let _ = scroll_view_down::document_impl(&mut doc, amount);
        assert_eq!(expected_text.to_string(), doc.client_view.text(&text));
        assert_eq!(expected_view, doc.client_view);
    }
    fn test_error(semantics: CursorSemantics, text: &str, view: View, amount: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_view(view);
        assert!(scroll_view_down::document_impl(&mut doc, amount).is_err());
    }

    #[test] fn scroll_down(){
        test(
            CursorSemantics::Block,
            "idk\nsome\nshit\n", 
            View::new(0, 0, 2, 2), 1, 
            "so\nsh\n", 
            View::new(0, 1, 2, 2), 
        );
        test(
            CursorSemantics::Bar,
            "idk\nsome\nshit\n", 
            View::new(0, 0, 2, 2), 1, 
            "so\nsh\n", 
            View::new(0, 1, 2, 2), 
        );
    }
    //TODO: test when amount > space left to scroll.    //does this saturate at doc bounds currently?

    #[test] fn errors_if_already_scrolled_down_all_the_way(){
        test_error(
            CursorSemantics::Block,
            "idk\nsome\nshit\n", 
            View::new(0, 2, 2, 2), 1, 
        );
        test_error(
            CursorSemantics::Bar,
            "idk\nsome\nshit\n", 
            View::new(0, 2, 2, 2), 1, 
        );
    }

    #[test] fn errors_if_amount_is_zero(){
        test_error(
            CursorSemantics::Block,
            "idk\nsome\nshit\n", 
            View::new(0, 1, 2, 2), 0, 
        );
        test_error(
            CursorSemantics::Bar,
            "idk\nsome\nshit\n", 
            View::new(0, 1, 2, 2), 0, 
        );
    }
}
