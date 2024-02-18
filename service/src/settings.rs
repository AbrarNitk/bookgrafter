pub struct GeminiSettings {
    pub(crate) key: String,
}

pub struct Settings {
    pub bind: String,
    pub port: u16,
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
    pub fn new() -> Self {
        Settings {
            bind: from_env_or("BIND", || "0.0.0.0".to_string()),
            port: from_env_or("PORT", || "8000".to_string())
                .parse()
                .expect("unexpected `PORT` value"),
            gemini: GeminiSettings {
                key: from_env("GEMINI_KEY"),
            },
        }
    }
}
