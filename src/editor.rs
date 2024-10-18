use crate::document::Document;
use crate::id::ClientID;
use std::{collections::HashMap, error::Error, path::PathBuf};



/// An Editor holds documents, and handles document interactions.
#[derive(Default)]
pub struct Editor{
    documents: HashMap<ClientID, Document>, //how could this be made to handle multiple clients manipulating the same document?
}
impl Editor{
    /// Returns a reference to the document associated with ClientID, if one exists.
    pub fn document(&self, client_id: ClientID) -> Option<&Document>{
        if let Some(doc) = self.documents.get(&client_id){
            return Some(doc);
        }

        None
    }
    /// Returns a mutable reference to the document associated with ClientID, if one exists.
    pub fn document_mut(&mut self, client_id: ClientID) -> Option<&mut Document>{
        if let Some(doc) = self.documents.get_mut(&client_id){
            return Some(doc);
        }

        None
    }
    /// Attempts to open specified document, and associate it with ClientID.
    pub fn open_document(&mut self, path: &PathBuf, client_id: ClientID) -> Result<(), Box<dyn Error>>{
        let doc = Document::open(path, crate::selection::CursorSemantics::Bar)?;
        //self.documents.insert(client_address.to_string(), doc);
        self.documents.insert(client_id, doc);

        Ok(())
    }
    /// Removes the document associated with ClientID, if one exists.
    // TODO: if doc is associated with multiple client ids, remove association with this ClientID
    pub fn close_document(&mut self, client_id: ClientID){
        if self.documents.contains_key(&client_id){
            self.documents.remove(&client_id);
        }
    }
}
