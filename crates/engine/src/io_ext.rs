//! Format crates registered with the I/O dispatcher.

use wordz_doc::Document;

/// Open a format other than plain text / JSON.
pub fn open(ext: &str, bytes: &[u8]) -> Option<Result<Document, String>> {
    match ext {
        "docx" | "docm" | "dotx" | "dotm" => Some(wordz_docx::read(bytes).map_err(|e| e.to_string())),
        "doc" | "dot" => Some(wordz_docbin::read(bytes).map_err(|e| e.to_string())),
        // Word XML Document (Flat OPC) or Word 2003 XML, told apart by the root element.
        "xml" => Some(wordz_docx::read_xml(bytes).map_err(|e| e.to_string())),
        // HTML, Markdown, ODT, RTF…: pictures fit the text width (see `io::fit_pictures`).
        other => wordz_formats::import(other, bytes).map(|r| {
            r.map(|mut doc| {
                crate::io::fit_pictures(&mut doc);
                doc
            })
        }),
    }
}

/// Save to a format other than plain text / JSON.
pub fn save(ext: &str, doc: &Document, opts: &crate::io::ExportOptions) -> Option<Result<Vec<u8>, String>> {
    // .docx/.docm/.dotx/.dotm: the extension picks the package flavour Word expects.
    if let Some(flavor) = wordz_docx::Flavor::from_ext(ext) {
        return Some(wordz_docx::write_as(doc, flavor).map_err(|e| e.to_string()));
    }
    match ext {
        "xml" if opts.word2003 => Some(wordz_docx::write_wordml2003(doc).map_err(|e| e.to_string())),
        "xml" => Some(wordz_docx::write_flat(doc).map_err(|e| e.to_string())),
        "png" => Some(render_png(doc, 0, 2.0, opts)),
        "pdf" => {
            let pdf = wordz_pdf::PdfOptions { include_markup: opts.markup, comments: opts.markup && opts.comments, ..Default::default() };
            Some(wordz_pdf::export(doc, &pdf).map_err(|e| e.to_string()))
        }
        other => wordz_formats::export_with(other, doc, &wordz_formats::ExportOptions { comments: opts.markup && opts.comments }),
    }
}

/// Render page `page` as PNG at `scale` px/pt. With Print Markup's comments the page has its
/// markup area with the comments in balloons (page images always show tracked changes).
pub fn render_png(doc: &Document, page: usize, scale: f32, opts: &crate::io::ExportOptions) -> Result<Vec<u8>, String> {
    let lo = wordz_layout::LayoutOptions { comment_balloons: opts.markup && opts.comments, ..Default::default() };
    let l = wordz_layout::layout(doc, &mut wordz_layout::LayoutCache::new(), &lo);
    let p = l.pages.get(page).ok_or_else(|| format!("no page {}", page + 1))?;
    let img = wordz_render::render_page(doc, p, scale, &Default::default());
    Ok(img.to_png())
}
