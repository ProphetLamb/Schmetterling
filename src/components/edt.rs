use comrak::{markdown_to_html, ComrakOptions};
use gloo_storage::*;
use web_sys::*;
use yew::prelude::*;

pub enum Markup {
    Html(String),
    Markdown(String),
}

impl Markup {
    pub fn with_text(&self, text: String) -> Markup {
        match self {
            Markup::Html(_) => Markup::Html(text),
            Markup::Markdown(_) => Markup::Markdown(text),
        }
    }
}

impl Default for Markup {
    fn default() -> Self {
        Self::Markdown(Default::default())
    }
}

impl ToString for Markup {
    fn to_string(&self) -> String {
        match self {
            Markup::Html(text) => text.to_owned(),
            Markup::Markdown(text) => text.to_owned(),
        }
    }
}

pub enum Msg {
    View,
    Edit,
    Update(String),
}

#[derive(PartialEq, Eq)]
pub enum State {
    View,
    Edit,
}

pub struct Edt {
    state: State,
    markup: Markup,
    html: String,
}

impl Edt {
    pub fn is_view(&self) -> bool {
        self.state == State::View
    }

    fn markup_to_html(&self) -> String {
        match &self.markup {
            Markup::Html(text) => text.to_owned(),
            Markup::Markdown(text) => markdown_to_html(text.as_str(), &ComrakOptions::default()),
        }
    }
}

impl Component for Edt {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            state: State::View,
            markup: Default::default(),
            html: Default::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let link = ctx.link();
        if self.is_view() {
            let edit_click = link.callback(|_| Msg::Edit);

            html! {
                <div>
                    <button onclick={edit_click}>{"Edit"}</button>
                    {self.html.to_owned()}
                </div>
            }
        } else {
            let view_click = link.callback(|_| Msg::View);
            let text_change = link.batch_callback(|e: Event| {
                e.target_dyn_into::<HtmlTextAreaElement>()
                    .map(|area| area.value())
                    .map(Msg::Update)
            });

            html! {
                <div>
                    <button onclick={view_click}>{"View"}</button>
                    <textarea onchange={text_change}>{self.text.text()}</textarea>
                </div>
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::View => {
                self.state = State::View;
                // Render the markup to html
                let html = self.markup_to_html();
                if html != self.html {
                    self.html = html;
                    true
                } else {
                    false
                }
            }
            Msg::Edit => {
                self.state = State::Edit;
                true
            }
            Msg::Update(markup) => {
                self.markup = self.markup.with_text(markup);

                false
            }
        }
    }
}
