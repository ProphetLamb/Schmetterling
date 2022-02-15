use std::fmt::{Display, Formatter};

use pulldown_cmark::{html::push_html, *};
use yew::prelude::*;

use crate::components::*;

#[derive(PartialEq, Eq)]
pub enum Markup {
    Html(String),
    Markdown(String),
}

impl Display for Markup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (name, markup) = match self {
            Markup::Html(markup) => ("Html", markup.as_str()),
            Markup::Markdown(markup) => ("Markdown", markup.as_str()),
        };

        write!(f, "{}({})", name, markup)
    }
}

impl Markup {
    pub fn markup(&self) -> &String {
        match self {
            Markup::Html(text) => text,
            Markup::Markdown(text) => text,
        }
    }

    pub fn with_text(&self, markup: String) -> Markup {
        match self {
            Markup::Html(_) => Markup::Html(markup),
            Markup::Markdown(_) => Markup::Markdown(markup),
        }
    }

    pub fn to_html(&self) -> Html {
        let html = match self {
            Markup::Html(html) => html.to_owned(),
            Markup::Markdown(md) => {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_TABLES);
                options.insert(Options::ENABLE_FOOTNOTES);
                options.insert(Options::ENABLE_STRIKETHROUGH);
                options.insert(Options::ENABLE_TASKLISTS);

                let parser = Parser::new_ext(md, options);
                let mut md_parsed = String::with_capacity(md.len());
                push_html(&mut md_parsed, parser);
                md_parsed
            }
        };
        html!(<RawHtml inner_html={html}/>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_markdown_dom() {
        let _html = Markup::Html("Placeholder".to_string()).to_html();
    }
}
