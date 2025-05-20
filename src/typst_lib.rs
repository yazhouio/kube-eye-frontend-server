use snafu::ResultExt;
use typst::{Document, layout::PagedDocument, syntax::Source};
use typst_as_library::TypstWrapperWorld;
use typst_pdf::PdfOptions;

use crate::{ error::{Error, Result, TypstPdfSnafu}};
static ROOT: &str = "./assets/templates";

pub fn generate_pdf_new(content: String) -> Result<Vec<u8>> {
    let world = TypstWrapperWorld::new("./assets".to_owned(), content.to_owned());
    let pdf = typst::compile(&world)
        .output
        .and_then(|d| typst_pdf::pdf(&d, &PdfOptions::default()));
    match pdf {
        Ok(pdf) => Ok(pdf),
        Err(e) => TypstPdfSnafu {
            message: format!("{:?}", e),
        }
        .fail(),
    }
}
