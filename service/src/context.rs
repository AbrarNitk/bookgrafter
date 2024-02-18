#[derive(Clone)]
pub struct Ctx {
    pub(crate) root: camino::Utf8PathBuf,
    pub(crate) gemini_key: String,
}

pub fn new(settings: &service::settings::Settings) -> Ctx {
    Ctx {
        root: settings.root.clone(),
        gemini_key: settings.gemini.key.to_owned(),
    }
}
