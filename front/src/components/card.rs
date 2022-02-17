use closure::closure;
use std::rc::Rc;
use web_sys::{HtmlInputElement, Node};
use yew::prelude::*;

use super::edt::{Edt, Presentation};
use crate::markup::*;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id: i64,
    #[prop_or_default]
    pub on_title_change: Callback<(i64, String, Event)>,
    #[prop_or_default]
    pub on_content_change: Callback<(i64, Markup, Event)>,
    #[prop_or_default]
    pub on_mode_change: Callback<(i64, Presentation)>,
    #[prop_or_default]
    pub on_click: Callback<i64>,
    #[prop_or_default]
    pub on_double_click: Callback<i64>,
}

enum Action {
    Mode(Presentation),
    Content(Markup),
    Title(String),
}

#[derive(PartialEq, Clone, Debug)]
struct State {
    mode: Presentation,
    content: Markup,
    title: String,
    id: i64,
}

impl State {
    fn with_mode(&self, mode: Presentation) -> Self {
        Self {
            mode,
            content: self.content.clone(),
            title: self.title.clone(),
            id: self.id,
        }
    }
    fn with_content(&self, value: Markup) -> Self {
        Self {
            mode: self.mode,
            content: value,
            title: self.title.clone(),
            id: self.id,
        }
    }
    fn with_title(&self, title: String) -> Self {
        Self {
            mode: self.mode,
            content: self.content.clone(),
            title,
            id: self.id,
        }
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Mode(mode) => {
                if self.mode == mode {
                    self
                } else {
                    self.with_mode(mode).into()
                }
            }
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
        id: props.id,
    });
    let id = props.id;
    let mode_cb = &props.on_mode_change;
    let view = Callback::from(closure!(clone state, clone mode_cb, |_| {
        state.dispatch(Action::Mode(Presentation::View));
        mode_cb.emit((id, Presentation::View));
    }));
    let edit = Callback::from(closure!(clone state, clone mode_cb, |_| {
        state.dispatch(Action::Mode(Presentation::Edit));
        mode_cb.emit((id, Presentation::Edit));
    }));
    let content = &props.on_content_change;
    let content = Callback::from(
        closure!(clone state, clone content, |(markup, e): (Markup, Event)| {
            content.emit((id, markup.clone(), e));
            state.dispatch(Action::Content(markup));
        }),
    );
    let title = &props.on_title_change;
    let title = closure!(clone state, clone title, |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().expect("Target must be HtmlInputElement.");
        title.emit((id, input.value(), e));
        state.dispatch(Action::Title(input.value()));
    });
    let click = &props.on_click;
    let click = closure!(clone click, |_| click.emit(id));
    let double_click = &props.on_double_click;
    let double_click = closure!(clone double_click, |_| double_click.emit(id));

    html! {
    <div class="Card" onclick={click} ondblclick={double_click}>
    if state.mode == Presentation::View {
        <div class="card-header">
            <span class="card-title">{state.title.clone()}</span>
            <button class="Icon" onclick={edit}>
                <i class="fa fa-pen" role="img" ></i>
            </button>
        </div>
    } else {
        <input class="card-header form-control" type="text" value={state.title.clone()} onchange={title} />
    }
        <Edt edit_classes={classes!("card-body", "form-control")} view_classes={classes!("card-body")} mode={state.mode} value={state.content.clone()} on_change={content} />
    if state.mode == Presentation::Edit {
        <div class="card-footer">
            <button class="btn-sm btn-primary" onclick={view}>{"Update"}</button>
        </div>
    }
    </div>
    }
}
