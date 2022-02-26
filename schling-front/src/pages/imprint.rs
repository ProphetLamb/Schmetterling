use schling_common::markup::Markup;
use yew::prelude::*;

#[function_component(Imprint)]
pub fn imprint() -> Html {
    let readme = include_str!("../../README.md");
    let readme = Markup::md_str(readme);
    html! {
        {readme.to_dom()}
    }
}
