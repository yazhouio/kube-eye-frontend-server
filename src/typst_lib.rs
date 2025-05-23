use std::path::PathBuf;

use snafu::OptionExt;
use typst_as_lib::TypstEngine;

use crate::{
    config::{Theme, TypstConfig},
    error::{InvalidInputSnafu, Result, TypstPdfSnafu},
};

pub fn generate_pdf(content: String, config: &TypstConfig, theme: &str) -> Result<Vec<u8>> {
    let root_path = PathBuf::from(&config.assets_dir);
    let theme_path = root_path.join(theme);
    let theme: &Theme = config.themes.get(theme).context(InvalidInputSnafu {
        reason: format!("Theme {} not found", theme),
    })?;

    let fonts: Vec<Vec<u8>> = theme
        .icons
        .iter()
        .filter_map(|font| {
            let font_path = config.icons.get(font).and_then(|p| {
                let p = root_path.join(p);
                if !p.exists() {
                    return None;
                }
                Some(p)
            });
            let Some(path) = font_path else {
                tracing::debug!("Failed to find font: {}", font);
                return None;
            };
            let Ok(bytes) = std::fs::read(path) else {
                tracing::debug!("Failed to read font: {}", font);
                return None;
            };
            Some(bytes)
        })
        .collect();
    let mut templates: Vec<(String, String)> = theme
        .themplates
        .iter()
        .filter_map(|(template_name, template_path)| {
            let path = theme_path.join(template_path);
            let Ok(temp) = std::fs::read_to_string(&path) else {
                tracing::debug!("Failed to read template: {}, path: {:?}", template_name, &path);
                return None;
            };
            Some((template_name.to_owned(), temp))
        })
        .collect();
    templates.push(("main.typ".to_string(), content));
    let templates = templates
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect::<Vec<(&str, &str)>>();

    let engine = TypstEngine::builder()
        .fonts(fonts)
        .with_static_source_file_resolver(templates)
        .build();

    let options = Default::default();

    engine.compile("main.typ").output.map_or_else(
        |e| {
            TypstPdfSnafu {
                message: format!("typst::compile() returned an error! {:?}", e),
            }
            .fail()
        },
        |d| match typst_pdf::pdf(&d, &options) {
            Ok(pdf) => Ok(pdf),
            Err(e) => TypstPdfSnafu {
                message: format!("Could not generate pdf. {:?}", e),
            }
            .fail(),
        },
    )

    // let pdf = typst_pdf::pdf(&doc, &options).expect("Could not generate pdf.");
}

// pub fn generate_pdf_new(content: String, assets_dir: &str) -> Result<Vec<u8>> {
//     let world = TypstWrapperWorld::new(assets_dir.to_owned(), content.to_owned());
//     let pdf = typst::compile(&world)
//         .output
//         .and_then(|d| typst_pdf::pdf(&d, &PdfOptions::default()));
//     match pdf {
//         Ok(pdf) => Ok(pdf),
//         Err(e) => TypstPdfSnafu {
//             message: format!("{:?}", e),
//         }
//         .fail(),
//     }
// }
