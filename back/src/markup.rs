use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Markup {
    pub text: String,
    pub lang: MarkupLang,
}

impl Clone for Markup {
    fn clone(&self) -> Self {
        Self {
            text: self.text.to_owned(),
            lang: self.lang,
        }
    }
}

impl From<Markup> for String {
    fn from(val: Markup) -> Self {
        val.text
    }
}

impl Markup {
    pub fn new(text: String, lang: MarkupLang) -> Self {
        Self { text, lang }
    }

    pub fn html(text: String) -> Self {
        Self {
            text,
            lang: MarkupLang::Html,
        }
    }

    pub fn md(text: String) -> Self {
        Self {
            text,
            lang: MarkupLang::Md,
        }
    }

    pub fn html_str(text: &str) -> Self {
        Self {
            text: text.to_string(),
            lang: MarkupLang::Html,
        }
    }

    pub fn md_str(text: &str) -> Self {
        Self {
            text: text.to_string(),
            lang: MarkupLang::Md,
        }
    }
}

impl Display for Markup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.lang, self.text)
    }
}

#[derive(PartialEq, Debug, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum MarkupLang {
    Html,
    Md,
}

impl MarkupLang {
    pub fn with_text(self, text: String) -> Markup {
        Markup::new(text, self)
    }
}

impl Display for MarkupLang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lang = match self {
            MarkupLang::Html => "html",
            MarkupLang::Md => "md",
        };
        write!(f, "{}", lang)
    }
}

impl Default for Markup {
    fn default() -> Self {
        Self {
            text: String::default(),
            lang: MarkupLang::Md,
        }
    }
}
