use closure::closure;
use std::rc::Rc;
use yew::prelude::*;

use super::doc::Document;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    cards: ChildrenWithProps<Document>,
}

#[function_component(Project)]
pub fn proj(props: &Props) -> Html {
    html! {
        {for props.cards.clone().into_iter()}
    }
}
