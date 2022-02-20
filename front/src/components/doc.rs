use closure::closure;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use yew::prelude::*;

use crate::{action, data::*, id, markup::Markup};

use super::{edt::*, sec};

#[derive(PartialEq, Debug, Clone)]
pub struct State {
    pub id: id::Doc,
    pub title: String,
    pub title_mode: Presentation,
    pub children: HashMap<id::Card, Card>,
}

impl State {
    pub fn card_next(&self) -> id::Card {
        id::Card {
            value: self.children.len() as u64,
            doc: self.id,
        }
    }

    fn own_title_view(&self, title: String) -> Self {
        Self {
            id: self.id,
            title,
            children: self.children.clone(),
            title_mode: Presentation::View,
        }
    }

    fn own_children(&self, children: HashMap<id::Card, Card>) -> Self {
        Self {
            id: self.id,
            title: self.title.clone(),
            children,
            title_mode: Presentation::View,
        }
    }

    fn own_mode(&self, title_mode: Presentation) -> Self {
        Self {
            id: self.id,
            title: self.title.clone(),
            children: self.children.clone(),
            title_mode,
        }
    }
}

impl Reducible for State {
    type Action = action::Doc;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            action::Doc::Add(id) => {
                let mut state = (*self).clone();
                state.children.insert(id, Card::with_id(id));
                doc_upd_children(state).into()
            }
            action::Doc::CardTitle((id, title, _)) => {
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
            action::Doc::CardContent((id, content, _)) => {
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
            action::Doc::Title(title) => {
                if self.title != title {
                    let state = self.own_title_view(title);
                    doc_upd_title(state).into()
                } else {
                    self
                }
            }
            action::Doc::TitleMode(mode) => self.own_mode(mode).into(),
        }
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

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id: id::Doc,
}

#[function_component(Document)]
pub fn doc(props: &Props) -> Html {
    let id = props.id;
    let state: UseReducerHandle<State> = use_reducer_eq(move || doc_get(id));

    let next_id = state.card_next();

    let title = Callback::from(closure!(clone state, |(title, _)|{
        state.dispatch(action::Doc::Title(title));
    }));
    let title_edit = Callback::from(closure!(clone state, |_|{
        state.dispatch(action::Doc::TitleMode(Presentation::Edit));
    }));
    let title_view = Callback::from(closure!(clone state, |_|{
        state.dispatch(action::Doc::TitleMode(Presentation::View));
    }));

    let add = Callback::from(closure!(clone state, |_| state.dispatch(action::Doc::Add(next_id))));
    let card_title =
        Callback::from(closure!(clone state, |p| state.dispatch(action::Doc::CardTitle(p))));
    let card_content =
        Callback::from(closure!(clone state, |p| state.dispatch(action::Doc::CardContent(p))));

    // Apply ordering to children
    let children = BTreeSet::<&Card>::from_iter(state.children.values());
    html! {
    <section class="Document">
        <div class="text-center">
            <TextEdit view_classes={classes!("display-5")} edit_classes={classes!("form-control", "display-5")} value={state.title.clone()} mode={state.title_mode} on_double_click={title_edit} on_change={title} on_blur={title_view} />
        </div>
        <div class="container">
        {
            for children.iter().map(|card| html!{
            <sec::Section
                id={card.id}
                title={card.title.clone()}
                content={card.content.clone()}
                on_title_change={card_title.clone()}
                on_content_change={card_content.clone()} />
            })
        }
        </div>
        <div class="d-flex m-3 justify-content-center">
            <button class="btn btn-circle" onclick={add}>
                <i class="fa fa-solid fa-plus fa-xl"></i>
            </button>
        </div>
    </section>
    }
}
