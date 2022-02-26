use yew::prelude::*;

use schling_common::id;

use crate::components::sec_list::SecList;

#[function_component(Home)]
pub fn home() -> Html {
    let id = id::Doc::new(0, id::Proj::new(0));
    html! {
    <div class="container">
        <SecList {id} />
    </div>
    }
}
