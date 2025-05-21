use typst_as_library::TypstWrapperWorld;
use typst_pdf::PdfOptions;

use crate::error::{Result, TypstPdfSnafu};

pub fn generate_pdf_new(content: String, assets_dir: &str) -> Result<Vec<u8>> {
    let world = TypstWrapperWorld::new(assets_dir.to_owned(), content.to_owned());
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
