#[derive(Debug)]
pub struct Error {
    pub details: String,
}

impl From<butane::Error> for Error {
    fn from(e: butane::Error) -> Self {
        Self {
            details: e.to_string(),
        }
    }
}
