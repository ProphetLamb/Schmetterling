use crate::markup::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(PartialEq)]
pub enum Msg {
    View,
    Edit,
    UpdTitle(String),
    UpdContent(String),
    Prop,
}

#[derive(PartialEq)]
pub enum State {
    View,
    Edit,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub key: String,
    #[prop_or(Markup::Markdown("Title".to_string()))]
    pub title: Markup,
    #[prop_or(Markup::Markdown("Content".to_string()))]
    pub content: Markup,
}

pub struct Edt {
    props: Props,
    title: Html,
    content: Html,
    state: State,
}

impl Edt {
    /// Whether the component of the current state accepts a specific message.
    pub fn accepts_msg(&self, msg: &Msg) -> bool {
        match msg {
            Msg::View => self.state == State::Edit,
            Msg::Edit => self.state == State::View,
            Msg::UpdContent(_) | Msg::UpdTitle(_) => self.state == State::Edit,
            Msg::Prop => true,
        }
    }
}

impl Component for Edt {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            title: props.title.to_dom(),
            content: props.content.to_dom(),
            props: props.clone(),
            state: State::View,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if !self.accepts_msg(&msg) {
            return false;
        }

        let props = &self.props;

        match msg {
            Msg::View => {
                self.title = props.title.to_dom();
                self.content = props.content.to_dom();
                self.state = State::View;
                true
            }
            Msg::Edit => {
                self.state = State::Edit;
                true
            }
            Msg::UpdContent(markup) => {
                self.props.content = props.content.with_text(markup);
                false
            }
            Msg::UpdTitle(markup) => {
                log::warn!("UpdTitle {}", markup);
                self.props.title = props.title.with_text(markup);
                false
            }
            Msg::Prop => {
                self.props = ctx.props().clone();
                ctx.link().callback(move |_: Event| Msg::View);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        match self.state {
            State::View => {
                let state_click = link.callback(move |_| Msg::Edit);
                html! {
                <div class="card border-secondary">
                <div class="card-header bg-transparent">
                <div class="d-flex justify-content-between flex-row flex-wrap">
                    <span class="card-title css-truncate css-truncate-overflow">{self.title.to_owned()}</span>
                    <div class="d-flex ml-3">
                    <svg onclick={state_click} class="mr-3" aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true">
                    <path fill-rule="evenodd" d="M11.013 1.427a1.75 1.75 0 012.474 0l1.086 1.086a1.75 1.75 0 010 2.474l-8.61 8.61c-.21.21-.47.364-.756.445l-3.251.93a.75.75 0 01-.927-.928l.929-3.25a1.75 1.75 0 01.445-.758l8.61-8.61zm1.414 1.06a.25.25 0 00-.354 0L10.811 3.75l1.439 1.44 1.263-1.263a.25.25 0 000-.354l-1.086-1.086zM11.189 6.25L9.75 4.81l-6.286 6.287a.25.25 0 00-.064.108l-.558 1.953 1.953-.558a.249.249 0 00.108-.064l6.286-6.286z"></path>
                    </svg>
                    </div>
                </div>
                </div>
                <div class="card-body">
                    {self.content.to_owned()}
                </div>
                </div>
                }
            }
            State::Edit => {
                let title_change = link.batch_callback(move |e: Event| {
                    e.target_dyn_into::<HtmlInputElement>()
                        .map(|area| Msg::UpdTitle(area.value()))
                });
                let content_change = link.batch_callback(move |e: Event| {
                    e.target_dyn_into::<HtmlTextAreaElement>()
                        .map(|area| Msg::UpdContent(area.value()))
                });
                let state_click = link.callback(move |_| Msg::View);
                html! {
                <div class="card border-primary">
                <div class="card-header bg-transparent">
                    <input type="text" class="card-title form-control" onchange={title_change} text={self.props.title.markup().to_owned()} />
                </div>
                <div class="card-body d-flex" style="flex-direction: column">
                <textarea class="form-control" onchange={content_change}>{self.props.content.markup().to_owned()}</textarea>
                <div class="d-flex justify-content-end mt-3">
                <button class="btn btn-primary" onclick={state_click}>{"Submit"}</button>
                </div>
                </div>
                </div>
                }
            }
        }
    }
}
