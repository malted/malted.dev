use crate::paper_page;
use tiny_http::Request;

static BODY: &str = include_str!("../../include/in.txt");

pub fn linkedin(request: Request) {
    paper_page(request, "I really don't like LinkedIn", BODY, None);
}
