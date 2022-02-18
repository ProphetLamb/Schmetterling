use closure::closure;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry::*;
use std::rc::Rc;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlDivElement, HtmlElement, HtmlInputElement, Location, Node};
use yew::prelude::*;

use super::doc;
use super::edt::{Edt, Presentation};
use crate::markup::*;

pub type Id = i64;

pub fn card_with(doc_id: doc::Id, card_id: Id) -> State {
    match doc::get_cards(doc_id).entry(card_id) {
        Occupied(entry) => entry.get().with_mode_view(),
        Vacant(entry) => entry.insert(State::default()).with_mode_view(),
    }
}

type ChangeEvent = Callback<(Id, State, Event)>;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub card_id: Id,
    pub doc_id: doc::Id,
    #[prop_or_default]
    pub on_change: ChangeEvent,
    #[prop_or_default]
    pub on_click: Callback<Id>,
    #[prop_or_default]
    pub on_double_click: Callback<Id>,
}

pub enum Action {
    Mode(Presentation),
    Content(Markup),
    Title(String),
    DoubleClick(MouseEvent),
    Blur(FocusEvent),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct State {
    mode: Presentation,
    content: Markup,
    title: String,
}

impl Default for State {
    fn default() -> Self {
        State {
            mode: Presentation::View,
            content: Markup::default(),
            title: "Section".to_string(),
        }
    }
}

impl State {
    fn with_mode_view(&self) -> Self {
        self.with_mode(Presentation::View)
    }

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
            Action::Mode(mode) => {
                if self.mode == mode {
                    self
                } else {
                    self.with_mode(mode).into()
                }
            }
            Action::Content(content) => self.with_content(content).into(),
            Action::Title(title) => self.with_title(title).into(),
            Action::DoubleClick(event) => {
                if self.mode == Presentation::View {
                    self.with_mode(Presentation::Edit).into()
                } else {
                    self
                }
            }
            Action::Blur(event) => {
                let target = event
                    .target_dyn_into::<HtmlElement>()
                    .expect_throw("Expected event target HtmlElement.");
                if let Some(card) = map_parent(target, |n| match n.dyn_into::<HtmlDivElement>() {
                    Ok(div) if div.class_list().contains("Card") => Ok(div),
                    Ok(div) => Err(div.into()),
                    Err(n) => Err(n.unchecked_into::<HtmlElement>()),
                }) {
                    if let Some(related) = event
                        .related_target()
                        .and_then(|et| et.dyn_into::<Node>().ok())
                    {
                        if card.contains(Some(&related)) {
                            return self;
                        }
                    }
                }
                let state = self.with_mode_view();
                state.into()
            }
        }
    }
}

fn map_parent<T: JsCast, F: Fn(HtmlElement) -> Result<T, HtmlElement>>(
    item: HtmlElement,
    select: F,
) -> Option<T> {
    let mut root = item;
    while let Some(child) = root
        .parent_node()
        .and_then(|node| node.dyn_into::<HtmlElement>().ok())
    {
        match select(child) {
            Ok(target) => return Some(target),
            Err(child) => root = child,
        }
    }
    None
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let doc_id = props.doc_id;
    let card_id = props.card_id;
    let state = use_reducer_eq(|| card_with(doc_id, card_id));

    let change = &props.on_change;
    let content = Callback::from(
        closure!(clone state, clone change, |(markup, e): (Markup, Event)| {
            state.dispatch(Action::Content(markup));
            change.emit((card_id, (*state).with_mode_view(), e));
        }),
    );
    let title = closure!(clone state, clone change, |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().expect("Target must be HtmlInputElement.");
        state.dispatch(Action::Title(input.value()));
        change.emit((card_id, (*state).with_mode_view(), e));
    });

    let click = &props.on_click;
    let click = closure!(clone click, |_| click.emit(card_id));

    let double_click = &props.on_double_click;
    let double_click = closure!(clone state, clone double_click, |e| {
        state.dispatch(Action::DoubleClick(e));
        double_click.emit(card_id);
    });

    let on_blur = closure!(clone state, clone change, |e: FocusEvent| {
        state.dispatch(Action::Blur(e.clone()));
        change.emit((card_id, (*state).clone(), e.into()))
    });

    html! {
    <div class="Card" id={format!("card-{}", card_id)} onclick={click} ondblclick={double_click} onblur={on_blur}>
        <div class="card-header">
    if state.mode == Presentation::View {
                    <i class="Icon fa fa-link"/>
                <span class="card-title">{state.title.clone()}</span>
        } else {
            <input class="form-control" type="text" value={state.title.clone()} onchange={title} />
        }
        </div>
        <Edt edit_classes={classes!("card-body", "form-control")} view_classes={classes!("card-body")} mode={state.mode} value={state.content.clone()} on_change={content} />
    </div>
    }
}
