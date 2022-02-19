use std::collections::HashMap;
use yew::prelude::*;

use super::doc;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Id {
    pub value: u64,
}

pub fn get_docs(id: Id) -> HashMap<doc::Id, doc::SerState> {
    crate::get_or_create(
        id.to_string().as_str(),
        HashMap::<doc::Id, doc::SerState>::new,
    )
}

pub fn upd_docs(id: Id, docs: HashMap<doc::Id, doc::SerState>) {
    crate::set(id.to_string().as_str(), docs);
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    children: Vec<doc::Id>,
}

#[function_component(Project)]
pub fn proj(props: &Props) -> Html {
    html! {
        <div>
        </div>
    }
}
