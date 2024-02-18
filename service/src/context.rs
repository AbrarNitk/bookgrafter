#[derive(Clone)]
pub struct Ctx {
    pub(crate) gemini_key: String,
}

pub fn new(settings: &service::settings::Settings) -> Ctx {
    Ctx {
        gemini_key: settings.gemini.key.to_owned(),
    }
}
