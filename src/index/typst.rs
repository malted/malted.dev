use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use std::path::Path;
use typst_as_lib::TypstEngine;

// Static font data
static FONTS: &[&[u8]] = &[
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-Bold.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-BoldItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-ExtraLight.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-ExtraLightItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-Italic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-Light.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-LightItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-Medium.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-MediumItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-Regular.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-SemiBold.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-SemiBoldItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-Thin.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Serif/IBMPlexSerif-ThinItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-Bold.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-BoldItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-ExtraLight.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-ExtraLightItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-Italic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-Light.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-LightItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-Medium.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-MediumItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-Regular.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-SemiBold.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-SemiBoldItalic.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-Thin.ttf"),
    include_bytes!("../../include/fonts/IBM_Plex_Mono/IBMPlexMono-ThinItalic.ttf"),
];

// Custom responder for PDF buffer
pub struct PdfResponse(Vec<u8>);

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for PdfResponse {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'static> {
        response::Response::build()
            .header(ContentType::PDF)
            .sized_body(self.0.len(), std::io::Cursor::new(self.0))
            .ok()
    }
}

#[get("/cv")]
pub fn cv(
    malted_state: &rocket::State<parking_lot::RwLock<crate::MaltedState>>,
) -> Result<PdfResponse, Status> {
    let template = std::fs::read_to_string(Path::new("include/hi.typ")).map_err(|e| {
        eprintln!("Typst template not found: {:?}", e);
        Status::InternalServerError
    })?;

    println!("location: {}", &malted_state.read().country);
    let template = template.replace("$LOCATION$", &malted_state.read().country);

    // Create the Typst engine with the template
    let template = TypstEngine::builder()
        .main_file(template)
        .fonts(FONTS.iter().copied())
        .with_package_file_resolver()
        .build();

    // Compile the document
    let compilation_result = template.compile();

    let document = match compilation_result.output {
        Ok(doc) => doc,
        Err(errors) => {
            eprintln!("Typst compilation failed: {:#?}", errors);
            return Err(Status::InternalServerError);
        }
    };

    // Generate PDF
    let options = Default::default();
    let pdf_bytes = typst_pdf::pdf(&document, &options).map_err(|e| {
        eprintln!("PDF generation failed: {:?}", e);
        Status::InternalServerError
    })?;

    println!("HAIIII");

    // Log any warnings
    for warning in &compilation_result.warnings {
        eprintln!("Typst warning: {:?}", warning);
    }

    Ok(PdfResponse(pdf_bytes))
}
