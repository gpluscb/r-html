pub use html_escape;
pub use r_html_codegen::template;

pub trait ToHtml {
    fn to_html(&self) -> String;
}

pub struct Html<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> ToHtml for Html<T> {
    fn to_html(&self) -> String {
        html_escape::encode_safe(&self.0).into_owned()
    }
}

pub struct RawHtml<T: ToString>(pub T);

impl<T: ToString> ToHtml for RawHtml<T> {
    fn to_html(&self) -> String {
        self.0.to_string()
    }
}
