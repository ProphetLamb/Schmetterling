use closure::closure;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry::*;
use std::collections::HashMap;
use yew::prelude::*;

use super::card;

pub type Id = i64;

pub fn get_key(doc_id: Id) -> String {
    format!("yew.schmetterling.doc.{:X}", doc_id)
}

pub fn get_cards(doc_id: Id) -> HashMap<card::Id, card::State> {
    let key = get_key(doc_id);
    crate::get_or_create(key.as_str(), HashMap::new)
}

pub fn upd_cards(doc_id: Id, cards: HashMap<card::Id, card::State>) {
    let key = get_key(doc_id);
    crate::set(key.as_str(), cards);
}

pub enum Action {
    Add(i64),
    Change((i64, card::State)),
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub doc_id: Id,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct State {
    doc_id: Id,
    title: String,
    children: Vec<i64>,
}

impl State {
    fn next_id(&self) -> Id {
        self.children.len() as Id
    }

    fn with_children(&self, children: Vec<Id>) -> Self {
        Self {
            doc_id: self.doc_id,
            title: self.title.clone(),
            children,
        }
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::Add(card_id) => {
                let mut children = self.children.clone();
                children.push(card_id);
                self.with_children(children).into()
            }
            Action::Change((card_id, new_state)) => {
                let mut cards = get_cards(self.doc_id);
                cards.insert(card_id, new_state);
                upd_cards(self.doc_id, cards);
                self
            }
        }
    }
}

#[function_component(Document)]
pub fn doc(props: &Props) -> Html {
    let doc_id = props.doc_id;
    let state = use_reducer_eq(move || {
        let cards = get_cards(doc_id);
        State {
            title: "Document".to_string(),
            children: cards.keys().into_iter().copied().collect(),
            doc_id,
        }
    });

    let next_id = state.next_id();

    let add = Callback::from(closure!(clone state, |_| state.dispatch(Action::Add(next_id))));
    let change = Callback::from(
        closure!(clone state, |(card_id, card_state, _)| state.dispatch(Action::Change((card_id, card_state)))),
    );

    html! {
    <section class="Document">
        <div class="text-center">
            <h1 class="display-5">{state.title.clone()}</h1>
        </div>
        <div class="container">
        { for state.children.iter().map(|card_id| html_nested!{
            <card::Card card_id={*card_id} {doc_id} on_change={change.clone()} />
        }) }
        </div>
        <div class="d-flex m-3">
            <button class="btn btn-circle" onclick={add}>
                <i class="fa fa-solid fa-plus fa-xl"></i>
            </button>
        </div>
    </section>
    }
}
