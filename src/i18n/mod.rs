//! Internationalization (i18n) Module
//! 
//! Provides multi-language support for the NativeHub UI.
//! Chinese (zh-CN) is the primary language.

mod strings;

pub use strings::*;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Lang {
    #[default]
    ZhCn, // 简体中文 - Default
    En,   // English
}

impl Lang {
    pub fn name(&self) -> &'static str {
        match self {
            Lang::ZhCn => "简体中文",
            Lang::En => "English",
        }
    }
    
    pub fn all() -> &'static [Lang] {
        &[Lang::ZhCn, Lang::En]
    }
}

/// Internationalization context
pub struct I18n {
    pub lang: Lang,
}

impl Default for I18n {
    fn default() -> Self {
        Self { lang: Lang::ZhCn }
    }
}

impl I18n {
    pub fn new(lang: Lang) -> Self {
        Self { lang }
    }
    
    /// Get translated string for a key
    pub fn t(&self, key: &str) -> &'static str {
        strings::get(self.lang, key)
    }
    
    /// Switch language
    pub fn set_lang(&mut self, lang: Lang) {
        self.lang = lang;
    }
}
