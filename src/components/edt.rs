use log::error;
use web_sys::HtmlTextAreaElement;
use yew::{prelude::*, virtual_dom::VNode};

use crate::markup::*;

#[derive(PartialEq, Eq)]
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

impl State {
    pub fn accepts_msg(&self, msg: &Msg) -> bool {
        match msg {
            Msg::View => self == &State::Edit,
            Msg::Edit => self == &State::View,
            Msg::Update(_) => self == &State::Edit,
        }
    }
}

pub struct Edt {
    markup: Markup,
    html: VNode,
    state: State,
}

impl Component for Edt {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            markup: Markup::Markdown("".to_string()),
            html: Markup::Html("Placeholder".to_string())
                .to_html()
                .expect("Placeholder to html"),
            state: State::View,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        match self.state {
            State::View => {
                let edit_click = link.callback(|_| Msg::Edit);
                html! {
                <div>
                    <button onclick={edit_click}>{"Edit"}</button>
                    <p>
                        {self.html.to_owned()}
                    </p>
                </div>
                }
            }
            State::Edit => {
                let view_click = link.callback(|_| Msg::View);
                let update_change = link.batch_callback(|e: Event| {
                    e.target_dyn_into::<HtmlTextAreaElement>()
                        .map(|area| Msg::Update(area.value()))
                });
                html! {
                <div>
                    <button onclick={view_click}>{"View"}</button>
                    <textarea onchange={update_change}>{self.markup.markup()}</textarea>
                </div>
                }
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::View => match self.markup.to_html() {
                Ok(html) => {
                    self.html = html;
                    true
                }
                Err(err) => {
                    error!("Failed to generate HTML DOM from {}. {}", self.markup, err);
                    false
                }
            },
            Msg::Edit => true,
            Msg::Update(markup) => {
                self.markup = self.markup.with_text(markup);
                false
            }
        }
    }
}
