pub struct GeminiSettings {
    pub(crate) key: String,
}

pub struct Settings {
    pub bind: String,
    pub port: u16,
    pub root: camino::Utf8PathBuf,
    pub(crate) gemini: GeminiSettings,
}

fn from_env(name: &str) -> String {
    match std::env::var(name) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("{}", format!("expected environment variable: {}", name));
            std::process::exit(1);
        }
    }
}

fn from_env_or<F: FnOnce() -> String>(name: &str, f: F) -> String {
    match std::env::var(name) {
        Ok(v) => v,
        Err(_) => f(),
    }
}

impl Settings {
    pub fn new(root: Option<camino::Utf8PathBuf>) -> Self {
        Settings {
            bind: from_env_or("BIND", || "0.0.0.0".to_string()),
            port: from_env_or("PORT", || "8000".to_string())
                .parse()
                .expect("unexpected `PORT` value"),
            root: create_root(root).expect("can not create the root"),
            gemini: GeminiSettings {
                key: from_env("GEMINI_KEY"),
            },
        }
    }
}

fn create_root(root: Option<camino::Utf8PathBuf>) -> std::io::Result<camino::Utf8PathBuf> {
    let root = match root {
        Some(r) => r.canonicalize_utf8()?,
        None => {
            camino::Utf8PathBuf::from_path_buf(std::env::current_dir()?.join("directory")).unwrap()
        }
    };
    std::fs::create_dir_all(root.as_path())?;
    Ok(root)
}
