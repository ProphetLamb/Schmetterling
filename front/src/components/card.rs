use closure::closure;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::edt::{Edt, Presentation};
use crate::markup::*;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {}

enum Action {
    Mode(Presentation),
    Content(Markup),
    Title(String),
}

#[derive(PartialEq, Debug)]
struct State {
    mode: Presentation,
    content: Markup,
    title: String,
}

impl State {
    fn with_mode(&self, mode: Presentation) -> Self {
        Self {
            mode,
            content: self.content.clone(),
            title: self.title.clone(),
        }
    }
    fn with_content(&self, value: Markup) -> Self {
        Self {
            mode: self.mode,
            content: value,
            title: self.title.clone(),
        }
    }
    fn with_title(&self, title: String) -> Self {
        Self {
            mode: self.mode,
            content: self.content.clone(),
            title,
        }
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Mode(mode) => self.with_mode(mode).into(),
            Action::Content(content) => self.with_content(content).into(),
            Action::Title(title) => self.with_title(title).into(),
        }
    }
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let state = use_reducer_eq(|| State {
        mode: Presentation::View,
        content: Markup::md_str(""),
        title: "".to_string(),
    });

    let view_click = Callback::from(closure!(clone state, |_| {
        state.dispatch(Action::Mode(Presentation::View));
    }));
    let edit_click = Callback::from(closure!(clone state, |_| {
        state.dispatch(Action::Mode(Presentation::Edit));
    }));
    let content_change = Callback::from(closure!(clone state, |(markup, _)| {
        state.dispatch(Action::Content(markup));
    }));
    let title_change = Callback::from(closure!(clone state, |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().expect("Target must be input.");
        state.dispatch(Action::Title(input.value()));
    }));
    html! {
    <div class="card">
    <div class="card-header d-flex justify-content-between">
    if state.mode == Presentation::View {
        <span class="card-title">{state.title.clone()}</span>
        <button onclick={edit_click}>{"Edit"}</button>
    } else {
        <input type="text" class="card-title" value={state.title.clone()} onchange={title_change} />
        <button onclick={view_click}>{"View"}</button>
    }
    </div>
        <Edt mode={state.mode} value={state.content.clone()} on_change={content_change} />
    </div>
    }
}
