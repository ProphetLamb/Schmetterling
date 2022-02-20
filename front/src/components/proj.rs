use closure::closure;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{components::doc, data::*, id, MainRoute};

pub enum Action {
    Add(id::Doc),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub id: id::Proj,
    pub children: HashMap<id::Doc, doc::Decl>,
}

impl State {
    pub fn with_id(id: id::Proj) -> Self {
        Self {
            id,
            children: HashMap::default(),
        }
    }

    pub fn doc_next(&self) -> id::Doc {
        id::Doc {
            value: self.children.len() as u64,
            proj: self.id,
        }
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::Add(id) => {
                let mut state = (*self).clone();
                state.children.insert(id, doc::Decl::with_id(id));
                proj_upd(state).into()
            }
        }
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id: id::Proj,
}

#[function_component(Project)]
pub fn proj(props: &Props) -> Html {
    let id = props.id;
    let state = use_reducer(move || proj_get(id));

    let next_id = state.doc_next();

    let add = Callback::from(closure!(clone state, |_| state.dispatch(Action::Add(next_id))));

    // Apply ordering to children
    let children = BTreeSet::<&doc::Decl>::from_iter(state.children.values());
    html! {
        <div class="container">
            <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
            {
                for children.iter().map(|doc| {
                    html!{
                <div class="ProjCard">
                    <div class="card-header">
                        <Link<MainRoute> classes={classes!("card-title")} to={MainRoute::Document{doc: {doc.id} }}>{doc.title.clone()}</Link<MainRoute>>
                        <i class="Icon fa fa-solid fa-edit fa-sm "></i>
                    </div>
                    <span class="card-body">{doc.summary.to_dom()}</span>
                </div>
                }})
            }
            </div>
            <div class="col d-flex m-3 justify-content-center">
                <button class="btn btn-circle" onclick={add}>
                    <i class="fa fa-solid fa-plus fa-xl"></i>
                </button>
            </div>
        </div>
    }
}
