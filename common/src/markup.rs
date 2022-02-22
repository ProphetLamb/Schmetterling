#[cfg(feature = "yew")]
use gloo_console::error;
#[cfg(feature = "yew")]
use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
#[cfg(feature = "yew")]
use web_sys::Element;
#[cfg(feature = "yew")]
use yew::virtual_dom::{VNode, VTag, VText};
#[cfg(feature = "yew")]
use yew::{html, Classes, Component, Context, Html, NodeRef, Properties};

#[derive(Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Markup {
    pub text: String,
    pub lang: MarkupLang,
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

impl Default for MarkupLang {
    fn default() -> Self {
        MarkupLang::Md
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

impl Markup {
    /// Creates the VDOM representation of the `Markup`.
    #[cfg(feature = "yew")]
    pub fn to_dom(&self) -> Html {
        match self.lang {
            MarkupLang::Html => html!(<RawHtml inner_html={self.text.to_owned()}/>),
            MarkupLang::Md => render_markdown(self.text.as_str()),
        }
    }
}

#[cfg(feature = "yew")]
#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct RawHtmlProps {
    pub inner_html: String,
}

/// Embeds the content of the `inner_html` string into the VDOM.
#[cfg(feature = "yew")]
pub struct RawHtml {
    props: RawHtmlProps,
    node_ref: NodeRef,
}

#[cfg(feature = "yew")]
impl Component for RawHtml {
    type Message = ();
    type Properties = RawHtmlProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().to_owned(),
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unreachable!()
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let new_props = ctx.props();
        if self.props.ne(new_props) {
            self.props = new_props.to_owned();
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.node_ref.clone()}/>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let el = self.node_ref.cast::<Element>().unwrap();
        el.set_inner_html(&self.props.inner_html);
    }
}

/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs

/// Adds a class to the VTag.
/// You can also provide multiple classes separated by ascii whitespaces.
///
/// Note that this has a complexity of O(n),
/// where n is the number of classes already in VTag plus
/// the number of classes to be added.
#[cfg(feature = "yew")]
fn add_class(vtag: &mut VTag, class: impl Into<Classes>) {
    let mut classes: Classes = vtag
        .attributes
        .iter()
        .find(|(k, _)| *k == "class")
        .map(|(_, v)| Classes::from(v.to_owned()))
        .unwrap_or_default();
    classes.push(class);
    vtag.add_attribute("class", classes.to_string());
}

/// Renders a string of Markdown to HTML with the default options (footnotes
/// disabled, tables enabled).
#[cfg(feature = "yew")]
pub fn render_markdown(src: &str) -> Html {
    let mut elems = vec![];
    let mut spine = vec![];

    macro_rules! add_child {
        ($child:expr) => {{
            let l = spine.len();
            assert_ne!(l, 0);
            spine[l - 1].add_child($child);
        }};
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    for ev in Parser::new_ext(src, options) {
        match ev {
            Event::Start(tag) => {
                spine.push(make_tag(tag));
            }
            Event::End(tag) => {
                // TODO Verify stack end.
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap();
                if let Tag::CodeBlock(_) = tag {
                    let mut pre = VTag::new("pre");
                    pre.add_child(top.into());
                    top = pre;
                } else if let Tag::Table(aligns) = tag {
                    for r in top.children_mut().iter_mut().flat_map(|ch| ch.iter_mut()) {
                        if let VNode::VTag(ref mut vtag) = r {
                            for (i, c) in vtag
                                .children_mut()
                                .iter_mut()
                                .flat_map(|ch| ch.iter_mut())
                                .enumerate()
                            {
                                if let VNode::VTag(ref mut vtag) = c {
                                    match aligns[i] {
                                        Alignment::None => {}
                                        Alignment::Left => add_class(vtag, "text-left"),
                                        Alignment::Center => add_class(vtag, "text-center"),
                                        Alignment::Right => add_class(vtag, "text-right"),
                                    }
                                }
                            }
                        }
                    }
                } else if let Tag::TableHead = tag {
                    for c in top.children_mut().iter_mut().flat_map(|ch| ch.iter_mut()) {
                        if let VNode::VTag(ref mut vtag) = c {
                            // TODO
                            //                            vtag.tag = "th".into();
                            vtag.add_attribute("scope", "col");
                        }
                    }
                }
                if l == 1 {
                    elems.push(top);
                } else {
                    spine[l - 2].add_child(top.into());
                }
            }
            Event::Text(text) => add_child!(VText::new(text.to_string()).into()),
            Event::Rule => add_child!(VTag::new("hr").into()),
            Event::SoftBreak => add_child!(VText::new("\n").into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),
            _ => error!(format!("Unknown event: {:#?}", ev)),
        }
    }

    if elems.len() == 1 {
        VNode::VTag(Box::new(elems.pop().unwrap()))
    } else {
        html! {
            { for elems.into_iter() }
        }
    }
}

#[cfg(feature = "yew")]
fn make_tag(t: Tag) -> VTag {
    match t {
        Tag::Paragraph => VTag::new("p"),
        Tag::Heading(n, _, _) => VTag::new(n.to_string()),
        Tag::BlockQuote => {
            let mut el = VTag::new("blockquote");
            el.add_attribute("class", "blockquote");
            el
        }
        Tag::CodeBlock(code_block_kind) => {
            let mut el = VTag::new("code");

            if let CodeBlockKind::Fenced(lang) = code_block_kind {
                // Different color schemes may be used for different code blocks,
                // but a different library (likely js based at the moment) would be necessary to actually provide the
                // highlighting support by locating the language classes and applying dom transforms
                // on their contents.
                match lang.as_ref() {
                    "html" => el.add_attribute("class", "html-language"),
                    "rust" => el.add_attribute("class", "rust-language"),
                    "java" => el.add_attribute("class", "java-language"),
                    "c" => el.add_attribute("class", "c-language"),
                    _ => {} // Add your own language highlighting support
                };
            }

            el
        }
        Tag::List(None) => VTag::new("ul"),
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(ref start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start.to_string());
            el
        }
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => {
            let mut el = VTag::new("table");
            el.add_attribute("class", "table");
            el
        }
        Tag::TableHead => VTag::new("th"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "font-italic");
            el
        }
        Tag::Strong => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "font-weight-bold");
            el
        }
        Tag::Link(_link_type, ref href, ref title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href.to_string());
            let title = title.clone().into_string();
            if !title.is_empty() {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::Image(_link_type, ref src, ref title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src.to_string());
            let title = title.clone().into_string();
            if !title.is_empty() {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::FootnoteDefinition(ref _footnote_id) => VTag::new("span"), // Footnotes are not rendered as anything special
        Tag::Strikethrough => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "text-decoration-strikethrough");
            el
        }
    }
}
