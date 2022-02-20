use closure::closure;
use std::collections::BTreeSet;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{action, data, id, ProjRoute};
#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id: id::Proj,
}

#[function_component(Project)]
pub fn proj(props: &Props) -> Html {
    let id = props.id;
    let state = use_reducer(move || data::proj_get(id));

    let next_id = state.doc_next();

    let add = Callback::from(closure!(clone state, |_| state.dispatch(action::Proj::Add(next_id))));

    // Apply ordering to children
    let children = BTreeSet::<&data::ProjDoc>::from_iter(state.children.values());
    html! {
        <div class="container">
            <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
            {
                for children.iter().map(|doc| {
                    let id = doc.id;
                    html!{
                <div class="col card shadow-sm">
                    <span class="card-header card-title">{doc.title.clone()}</span>
                    <span class="card-body">{doc.summary.to_dom()}</span>
                    <div class="card-footer">
                        <Link<ProjRoute> classes={classes!("btn", "btn-light")} to={ProjRoute::Doc{proj: id.proj.value, doc: id.value}}>{"View"}</Link<ProjRoute>>
                    </div>
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
