use std::fmt::{Display, Formatter};

use js_sys::JsString;
use pulldown_cmark::{html::push_html, *};
use wasm_bindgen::{JsCast, JsStatic, JsValue};
use web_sys::{DomParser, SupportedType};
use yew::{virtual_dom::VNode, Html};

use crate::markup;

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

    pub fn to_html(&self) -> Result<Html, JsString> {
        // https://github.com/pvcresin/yew-markdown-preview/blob/master/src/app.rs
        let mut md_parsed;
        let html = match self {
            Markup::Html(html) => html,
            Markup::Markdown(md) => {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_TABLES);
                options.insert(Options::ENABLE_FOOTNOTES);
                options.insert(Options::ENABLE_STRIKETHROUGH);
                options.insert(Options::ENABLE_TASKLISTS);

                let parser = Parser::new_ext(md, options);
                md_parsed = String::with_capacity(md.len());
                push_html(&mut md_parsed, parser);
                &md_parsed
            }
        };
        let parser = DomParser::new().map_err(|v| safe_to_js_string(&v))?;
        let doc = DomParser::parse_from_string(&parser, html.as_str(), SupportedType::TextHtml)
            .map_err(|v| safe_to_js_string(&v))?;
        Ok(VNode::VRef(doc.get_root_node()))
    }
}

fn safe_to_js_string(value: &JsValue) -> JsString {
    value
        .dyn_ref()
        .unwrap_or(&JsString::from("unknown"))
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_markdown_dom() {
        let _html = Markup::Html("Placeholder".to_string())
            .to_html()
            .expect("Placeholder to html");
    }
}
