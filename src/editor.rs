use crate::document::Document;
use crate::id::ClientID;
use std::{collections::HashMap, error::Error, path::PathBuf};



/// An Editor holds documents. Document interactions are performed through the editor
#[derive(Default)]
pub struct Editor{
    documents: HashMap<ClientID, Document>,
}
impl Editor{
    /// Returns a reference to the document associated with ClientID, if one exists.
    pub fn document(&self, client_address: ClientID) -> Option<&Document>{
        if let Some(doc) = self.documents.get(&client_address){
            return Some(doc);
        }

        None
    }
    /// Returns a mutable reference to the document associated with ClientID, if one exists.
    pub fn document_mut(&mut self, client_address: ClientID) -> Option<&mut Document>{
        if let Some(doc) = self.documents.get_mut(&client_address){
            return Some(doc);
        }

        None
    }
    /// Attempts to open specified document, and associate it with ClientID.
    pub fn open_document(&mut self, path: &PathBuf, client_address: ClientID) -> Result<(), Box<dyn Error>>{
        let doc = Document::open(path, crate::selection::CursorSemantics::Bar)?;
        //self.documents.insert(client_address.to_string(), doc);
        self.documents.insert(client_address, doc);

        Ok(())
    }
    /// Removes the document associated with ClientID, if one exists.
    // TODO: if doc is associated with multiple client ids, remove association with this ClientID
    pub fn close_document(&mut self, client_address: ClientID){
        if self.documents.contains_key(&client_address){
            self.documents.remove(&client_address);
        }
    }
}
