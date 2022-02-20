use closure::closure;
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{data::*, id, markup::Markup};

use super::{sec, text::*};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Decl {
    pub id: id::Doc,
    pub title: String,
    pub summary: Markup,
    pub order: u64,
}

impl Ord for Decl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Decl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl Decl {
    pub fn with_id(id: id::Doc) -> Self {
        Self {
            id,
            title: format!("Document {}", id.value),
            summary: Markup::md_str(""),
            order: id.value,
        }
    }
}

pub type ChangeEvent<T> = Callback<(id::Doc, T)>;

#[derive(Properties, PartialEq, Debug)]
pub struct EditProps {
    decl: Decl,
    #[prop_or_default]
    on_title_change: ChangeEvent<String>,
    #[prop_or_default]
    on_summary_change: ChangeEvent<Markup>,
}

#[function_component(Edit)]
pub fn edit(props: &EditProps) -> Html {
    let decl = props.decl.clone();
    let id = decl.id;

    let title = props.on_title_change.clone();
    let title = Callback::from(move |e: Event| {
        let input = e
            .target_dyn_into::<HtmlInputElement>()
            .expect("event target is HtmlInputElement");
        title.emit((id, input.value()))
    });

    let summary = props.on_summary_change.clone();
    let summary = Callback::from(move |(value, _): (Markup, Event)| summary.emit((id, value)));

    html! {
    <>
        <div class="mb-3">
            <label for="title" class="form-label">{"Title"}</label>
            <input type="text" class="form-control" value={decl.title} id="title" onchange={title} />
        </div>
        <div class="mb-3">
            <label for="summary" class="form-label">{"Summary"}</label>
            <MarkupEdit edit_classes={classes!("form-control")} mode={Presentation::Edit} value={decl.summary} id="summary" on_change={summary}/>
        </div>
    </>
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: id::Card,
    pub title: String,
    pub content: Markup,
    pub order: u64,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl Card {
    fn with_id(id: id::Card) -> Self {
        Self {
            id,
            title: format!("Section {}", id.value),
            content: Markup::default(),
            order: id.value,
        }
    }
}

pub enum Action {
    Add(id::Card),
    CardTitle((id::Card, String, Event)),
    CardContent((id::Card, Markup, Event)),
    DeclMode(Presentation),
}

#[derive(PartialEq, Debug, Clone)]
pub struct State {
    pub id: id::Doc,
    pub decl: Decl,
    pub decl_mode: Presentation,
    pub children: HashMap<id::Card, Card>,
}

impl State {
    pub fn card_next(&self) -> id::Card {
        id::Card {
            value: self.children.len() as u64,
            doc: self.id,
        }
    }

    fn own_children(&self, children: HashMap<id::Card, Card>) -> Self {
        Self {
            id: self.id,
            decl: self.decl.clone(),
            children,
            decl_mode: Presentation::View,
        }
    }

    fn own_mode(&self, title_mode: Presentation) -> Self {
        Self {
            id: self.id,
            decl: self.decl.clone(),
            children: self.children.clone(),
            decl_mode: title_mode,
        }
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::Add(id) => {
                let mut state = (*self).clone();
                state.children.insert(id, Card::with_id(id));
                doc_upd_children(state).into()
            }
            Action::CardTitle((id, title, _)) => {
                if matches!(self.children.get(&id), Some(card) if card.title != title) {
                    let mut children = self.children.clone();
                    if let Some(card) = children.get_mut(&id) {
                        card.title = title;
                    }
                    let state = self.own_children(children);
                    doc_upd_children(state).into()
                } else {
                    self
                }
            }
            Action::CardContent((id, content, _)) => {
                if matches!(self.children.get(&id), Some(card) if card.content != content) {
                    let mut state = (*self).clone();
                    if let Some(card) = state.children.get_mut(&id) {
                        card.content = content;
                    }
                    doc_upd_children(state).into()
                } else {
                    self
                }
            }
            Action::DeclMode(mode) => self.own_mode(mode).into(),
        }
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id: id::Doc,
}

#[function_component(Document)]
pub fn doc(props: &Props) -> Html {
    let id = props.id;
    let state = use_reducer_eq(move || doc_get(id));

    let next_id = state.card_next();

    let decl_edit = Callback::from(
        closure!(clone state, |_| state.dispatch(Action::DeclMode(Presentation::Edit))),
    );
    let add = Callback::from(closure!(clone state, |_| state.dispatch(Action::Add(next_id))));

    html! {
    <>
    <section class="Document">
        <div class="text-center" ondblclick={decl_edit}>
            <span class="display-5">{state.decl.title.clone()}</span>
        </div>
        <div class="container">
    { render_children_ord(state.clone()) }
        </div>
        <div class="d-flex m-3 justify-content-center">
            <button class="btn btn-circle" onclick={add}>
                <i class="fa fa-solid fa-plus fa-xl"></i>
            </button>
        </div>
    </section>
    {render_decl_edit_modal(&*state)}
    </>
    }
}

fn render_decl_edit_modal(state: &State) -> Html {
    html! {
    <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog">
        <div class="modal-content">
        <div class="modal-header">
            <h5 class="modal-title" id="exampleModalLabel">{state.decl.title.clone()}</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
            <Edit decl={state.decl.clone()} />
        </div>
        <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
            <button type="button" class="btn btn-primary">{"Save changes"}</button>
        </div>
        </div>
    </div>
    </div>
    }
}

fn render_children_ord(state: UseReducerHandle<State>) -> Html {
    let seg_title = Callback::from(closure!(clone state, |p| state.dispatch(Action::CardTitle(p))));
    let seg_content =
        Callback::from(closure!(clone state, |p| state.dispatch(Action::CardContent(p))));
    // Apply ordering to children
    let children = BTreeSet::<&Card>::from_iter(state.children.values());

    html! {
    <>
    {
        for children.iter().map(|card| html!{
        <sec::Section
            id={card.id}
            title={card.title.clone()}
            content={card.content.clone()}
            on_title_change={seg_title.clone()}
            on_content_change={seg_content.clone()} />
        })}
    </>
    }
}
