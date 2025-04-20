use std::fs;
use std::error::Error;
use std::io::BufWriter;
use crate::document::Document;

/// Saves the document's content to its file path.
pub fn document_impl(document: &mut Document) -> Result<(), Box<dyn Error>>{
    if let Some(path) = &document.file_path{ // does nothing if path is None    //maybe return Err(()) instead?
        document.text.write_to(BufWriter::new(fs::File::create(path)?))?;
        //self.modified = false;    //old code. no longer used this way...
        document.last_saved_text = document.text.clone();
    }
    
    Ok(())
}

//not sure how to test this here. has been tested by using fn from frontend code...
