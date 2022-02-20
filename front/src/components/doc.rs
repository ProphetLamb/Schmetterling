use closure::closure;
use std::collections::BTreeSet;
use yew::prelude::*;

use crate::{action, data, id};

use super::{edt::*, sec};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id: id::Doc,
}

#[function_component(Document)]
pub fn doc(props: &Props) -> Html {
    let id = props.id;
    let state: UseReducerHandle<data::Doc> = use_reducer_eq(move || data::doc_get(id));

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
    let children = BTreeSet::<&data::DocCard>::from_iter(state.children.values());
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
