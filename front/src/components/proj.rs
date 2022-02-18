use closure::closure;
use std::{collections::HashMap, rc::Rc};
use yew::prelude::*;

use super::doc;

pub type Id = i64;

pub fn get_key(proj_id: Id) -> String {
    format!("yew.schmetterling.proj.{:X}", proj_id)
}

pub fn get_cards(proj_id: Id) -> HashMap<doc::Id, doc::State> {
    let key = get_key(proj_id);
    crate::get_or_create(key.as_str(), HashMap::new)
}

pub fn upd_cards(proj_id: Id, docs: HashMap<doc::Id, doc::State>) {
    let key = get_key(proj_id);
    crate::set(key.as_str(), docs);
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    cards: ChildrenWithProps<doc::Document>,
}

#[function_component(Project)]
pub fn proj(props: &Props) -> Html {
    html! {
        {for props.cards.clone().into_iter()}
    }
}
