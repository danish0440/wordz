//! Rich Text Format adapter using craft-rtf.

pub use craft_rtf::doc::esc;
pub use craft_rtf::{dttm_to_iso, iso_to_dttm};

/// Import RTF bytes into WordZ's document model.
pub fn import(bytes: &[u8]) -> Result<wordz_doc::Document, String> {
    let shared = craft_rtf::doc::import(bytes)?;
    wordz_docx::from_shared_model(shared).map_err(|e| e.to_string())
}

/// Export a WordZ document to RTF.
pub fn export(doc: &wordz_doc::Document) -> String {
    let Ok(shared) = wordz_docx::to_shared_model(doc) else {
        return String::new();
    };
    craft_rtf::doc::export(&shared)
}

/// Parse RTF bytes into the flow model.
pub fn parse(bytes: &[u8]) -> Result<crate::model::Flow, String> {
    let doc = import(bytes)?;
    Ok(crate::model::from_doc(&doc))
}
