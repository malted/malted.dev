use tiny_http::{Request, Response};
use typst::foundations::{Dict, IntoValue};
use typst_as_lib::TypstEngine;
use typst_as_lib::typst_kit_options::TypstKitFontOptions;

static CV_SRC_FILE: &str = include_str!("../../include/cv.typ");
static FONTS: &[&[u8]] = &[
    include_bytes!("../../include/fonts/NewCM08-Book.otf"),
    include_bytes!("../../include/fonts/NewCM08-BookItalic.otf"),
    include_bytes!("../../include/fonts/NewCM10-Bold.otf"),
    include_bytes!("../../include/fonts/NewCM10-BoldItalic.otf"),
];

pub fn cv(request: Request) {
    let url = url::Url::parse(&format!("http://localhost{}", request.url())).expect("a valid URL");
    let email = url
        .query_pairs()
        .find(|(key, _)| key == "email")
        .map(|(_, val)| val.to_string())
        .unwrap_or("malted@malted.dev".to_string());

    let pdf = compile(email);

    let response = Response::from_data(pdf);
    request.respond(response).unwrap();
}

fn compile(email: String) -> Vec<u8> {
    let template = TypstEngine::builder()
        .main_file(CV_SRC_FILE)
        .fonts(FONTS.iter().copied())
        .search_fonts_with(TypstKitFontOptions::default())
        .with_package_file_resolver()
        .build();

    let mut inputs = Dict::new();
    inputs.insert("email".into(), email.into_value());

    let compilation_result = template.compile_with_input(inputs);

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
