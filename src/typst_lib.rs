use snafu::ResultExt;
use typst::{Document, layout::PagedDocument, syntax::Source};
use typst_as_lib::TypstEngine;
use typst_as_library::TypstWrapperWorld;
use typst_pdf::PdfOptions;

use crate::error::{Error, Result, TypstCompileSnafu, TypstPdfSnafu};
static ROOT: &str = "./assets/templates";

pub fn generate_pdf(content: String) -> Result<Vec<u8>> {
    let doc = TypstEngine::builder()
        .with_file_system_resolver(ROOT)
        .main_file(Source::detached(content))
        .build()
        .compile()
        .output
        .context(TypstCompileSnafu)?;
    let pdf = typst_pdf::pdf(&doc, &PdfOptions::default());
    match pdf {
        Ok(pdf) => Ok(pdf),
        Err(e) => TypstPdfSnafu {
            message: format!("{:?}", e),
        }
        .fail(),
    }
}

pub fn generate_pdf_new(content: String) -> Result<Vec<u8>> {
    let content = r#"
#set text(
  font: "New Computer Modern",
  size: 10pt
)
#set page(
  paper: "a6",
  margin: (x: 1.8cm, y: 1.5cm),
)
#set par(
  justify: true,
  leading: 0.52em,
)

= Introduction
In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behavior of these natural structures.

#table(
  columns: (auto, auto, auto),
  inset: 10pt,
  align: center,
  [*姓名*], [*年龄*], [*职业*],
  [张三], [25], [工程师],
  [李四], [30], [设计师],
  [王五], [28], [教师],
)
"#
    .to_owned();
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
