use closure::closure;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};
use yew::prelude::*;

use super::{card, edt::*, proj};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Id {
    pub proj: proj::Id,
    pub value: u64,
}

pub fn get_doc(id: Id) -> State {
    crate::get_or_create(&format!("{}", id), || SerState::with_id(id)).into()
}

pub fn upd_doc(state: State) {
    let state: SerState = state.into();
    // Ensure only upload mode view.
    crate::set(&format!("{}", state.id), state);
}

pub enum Action {
    Add(card::Id),
    Card((card::Id, card::State)),
    Title(String),
    TitleMode(Presentation),
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id: Id,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct SerState {
    pub id: Id,
    pub title: String,
    pub children: HashMap<card::Id, card::SerState>,
}

impl SerState {
    fn with_id(id: Id) -> Self {
        Self {
            id,
            title: "Document".to_string(),
            children: HashMap::default(),
        }
    }
}

macro_rules! mapv_into {
    ($map:expr) => {
        $map.into_iter()
            .map(|(id, value)| (id, value.into()))
            .collect()
    };
}

impl From<State> for SerState {
    fn from(val: State) -> Self {
        SerState {
            id: val.id,
            title: val.title,
            children: mapv_into!(val.children),
        }
    }
}

impl From<SerState> for State {
    fn from(val: SerState) -> Self {
        State {
            id: val.id,
            title: val.title,
            title_mode: Presentation::View,
            children: mapv_into!(val.children),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct State {
    pub id: Id,
    pub title: String,
    pub title_mode: Presentation,
    pub children: HashMap<card::Id, card::State>,
}

impl State {
    fn next_id(&self) -> card::Id {
        card::Id {
            value: self.children.len() as u64,
            doc: self.id,
        }
    }

    fn with_children(&self, children: HashMap<card::Id, card::State>) -> Self {
        Self {
            id: self.id,
            title: self.title.clone(),
            children,
            title_mode: self.title_mode,
        }
    }

    fn with_title(&self, title: String) -> Self {
        Self {
            id: self.id,
            title,
            children: self.children.clone(),
            title_mode: self.title_mode,
        }
    }

    fn with_mode(&self, title_mode: Presentation) -> Self {
        Self {
            id: self.id,
            title: self.title.clone(),
            children: self.children.clone(),
            title_mode,
        }
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::Add(id) => {
                let mut children = self.children.clone();
                children.insert(id, card::State::default());
                self.with_children(children).into()
            }
            Action::Card((id, new_state)) => {
                let mut state = get_doc(self.id);
                state.children.insert(id, new_state);
                upd_doc(state.clone());
                state.into()
            }
            Action::Title(title) => {
                let state = self.with_title(title);
                upd_doc(state.clone());
                state.into()
            }
            Action::TitleMode(mode) => self.with_mode(mode).into(),
        }
    }
}

#[function_component(Document)]
pub fn doc(props: &Props) -> Html {
    let id = props.id;
    let state = use_reducer_eq(move || get_doc(id));

    let next_id = state.next_id();

    let title = Callback::from(closure!(clone state, |(title, _)|{
        state.dispatch(Action::Title(title));
    }));
    let title_edit = Callback::from(closure!(clone state, |_|{
        state.dispatch(Action::TitleMode(Presentation::Edit));
    }));
    let title_view = Callback::from(closure!(clone state, |_|{
        state.dispatch(Action::TitleMode(Presentation::View));
    }));

    let add = Callback::from(closure!(clone state, |_| state.dispatch(Action::Add(next_id))));
    let change = Callback::from(
        closure!(clone state, |(card_id, card_state, _)| state.dispatch(Action::Card((card_id, card_state)))),
    );

    html! {
    <section class="Document">
        <div class="text-center">
            <TextEdit view_classes={classes!("display-5")} edit_classes={classes!("display-5", "form-control")} value={state.title.clone()} mode={state.title_mode} on_double_click={title_edit} on_change={title} on_blur={title_view} />
        </div>
        <div class="container">
        { for state.children.keys().into_iter().map(|id| html_nested!{
            <card::Card id={*id} on_change={change.clone()} />
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
