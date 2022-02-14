pub mod edt;

use log::error;
use yew::prelude::*;

use crate::components::edt::*;
use crate::markup::*;

pub struct Home;

impl Component for Home {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <Edt></Edt>
        }
    }
}

pub struct Imprint;

const README: &str = include_str!("../../README.md");

impl Component for Imprint {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match Markup::Markdown(README.to_string()).to_html() {
            Ok(html) => html,
            Err(json) => {
                error!("{}", json);
                html! {}
            }
        }
    }
}

pub struct PageNotFound;

impl Component for PageNotFound {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="justify-center">
        <h1>{"404"}</h1>
        <p>{"This is not the page you are looking for."}</p>
        </div>
        }
    }
}
