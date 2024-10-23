use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct TranslationConfig {
    #[serde_inline_default(false)]
    pub enabled: bool,
    #[serde_inline_default(true)]
    pub client_translations: bool,
    #[serde_inline_default(None)]
    pub translation_file_path: Option<PathBuf>,
}

impl Default for TranslationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            client_translations: true,
            translation_file_path: None,
        }
    }
}