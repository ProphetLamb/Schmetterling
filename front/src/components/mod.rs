pub mod card;
pub mod edt;

use web_sys::Element;
use yew::prelude::*;

use crate::components::card::Card;
use crate::markup::*;

pub struct Home;

impl Component for Home {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
        <>
            <Card />
            <Card />
            <Card />
        </>
        }
    }
}

pub struct Imprint;

const README: &str = include_str!("../../README.md");

impl Component for Imprint {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Markup::md(README.to_string()).to_dom()
    }
}

pub struct PageNotFound;

impl Component for PageNotFound {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div class="justify-center">
            <h1>{"404"}</h1>
            <p>{"This is not the page you are looking for."}</p>
        </div>
        }
    }
}
