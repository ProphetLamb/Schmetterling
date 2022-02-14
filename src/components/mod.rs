use yew::prelude::*;

pub struct Header;

impl Component for Header {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <header class="p-3"><div class="container"><div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
                <a class="d-flex align-items-center mb-2 mb-lg-0 text-white text-decoration-none" href="/">
                    <img src="favicon.svg" title="Schmetterling"/>
                </a>
            </div></div></header>
        }
    }
}

pub struct Footer;

impl Component for Footer {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <footer class="footer mt-auto py-3 bg-light"><div class="container">
                <span class="text-muted">{"Copyright (c) 2022 ProphetLamb"}</span>
            </div></footer>
        }
    }
}

pub struct Imprint;

impl Component for Imprint {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1>{"Schmetterling"}</h1>
        }
    }
}
