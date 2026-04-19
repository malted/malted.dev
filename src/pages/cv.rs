use std::fs;
use tiny_http::{Request, Response};
use typst_as_lib::TypstEngine;
use typst_as_lib::typst_kit_options::TypstKitFontOptions;

static CV_SRC_FILE: &str = include_str!("../../include/cv.typ");
static FONTS: &[&[u8]] = &[
    include_bytes!("../../include/NewCM10-Regular.otf"),
    include_bytes!("../../include/fonts/NewCM08-Book.otf"),
    include_bytes!("../../include/fonts/NewCM08-BookItalic.otf"),
    include_bytes!("../../include/fonts/NewCM10-Bold.otf"),
];

pub struct PdfResponse(Vec<u8>);

pub fn cv(request: Request) {
    let pdf = compile();

    let response = Response::from_data(pdf);
    request.respond(response).unwrap();
}

fn compile() -> Vec<u8> {
    let template = TypstEngine::builder()
        .main_file(CV_SRC_FILE)
        .fonts(FONTS.iter().copied())
        .search_fonts_with(TypstKitFontOptions::default())
        .with_package_file_resolver()
        .build();

    let compilation_result = template.compile(); //_with_input(dummy_data())

    for warning in &compilation_result.warnings {
        eprintln!("Typst warning: {:?}", warning);
    }

    let document = compilation_result
        .output
        .expect("typst::compile() returned an error!");

    let options = Default::default();
    let pdf = typst_pdf::pdf(&document, &options).expect("Could not generate pdf.");

    pdf
}
