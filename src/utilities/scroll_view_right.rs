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

/// Returns a new instance of [`View`] with `horizontal_start` increased by specified amount.
/// # Errors
///     //if `amount` is 0.
///     //if function would return a `View` with the same state.
fn view_impl(view: &View, amount: usize, text: &Rope) -> Result<View, ViewError>{
    if amount == 0{return Err(ViewError::InvalidInput);}

    // TODO: cache longest as a field in [`View`] struct to eliminate having to calculate this on each call
    // Calculate the longest line width in a single pass
    let longest = text.lines()
        .map(|line| crate::text_util::line_width(line, false))
        .max()
        .unwrap_or(0); // Handle the case where there are no lines

    let new_horizontal_start = view.horizontal_start.saturating_add(amount);

    if new_horizontal_start + view.width <= longest{
        Ok(View::new(new_horizontal_start, view.vertical_start, view.width, view.height))
    }else{
        //Ok(self.clone())
        Err(ViewError::ResultsInSameState)
    }
}

#[cfg(test)]
mod tests{
    use crate::utilities::scroll_view_right;
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
        let _ = scroll_view_right::document_impl(&mut doc, amount);
        assert_eq!(expected_text.to_string(), doc.client_view.text(&text));
        assert_eq!(expected_view, doc.client_view);
    }
    fn test_error(text: &str, view: View, amount: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_view(view);
        assert!(scroll_view_right::document_impl(&mut doc, amount).is_err());
    }

    #[test] fn scroll_right(){
        test(
            "idk\nsome\nshit\n", 
            View::new(0, 0, 2, 2), 1, 
            "dk\nom\n", 
            View::new(1, 0, 2, 2), 
            CursorSemantics::Block
        );
    }
    //TODO: test when amount > space left to scroll.    //does this saturate at doc bounds currently?

    #[test] fn errors_if_already_scrolled_right_all_the_way(){
        test_error(
            "idk\nsome\nshit\n", 
            View::new(2, 0, 2, 2), 1, 
            CursorSemantics::Block
        );
    }

    #[test] fn errors_if_amount_is_zero(){
        test_error(
            "idk\nsome\nshit\n", 
            View::new(1, 0, 2, 2), 0, 
            CursorSemantics::Block
        );
    }
}