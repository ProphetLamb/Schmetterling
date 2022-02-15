#![recursion_limit = "1024"]

pub mod components;
pub mod markup;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/imprint")]
    Imprint,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn switch(&self) -> Html {
        match self {
            Route::Home => html! {<Home/>},
            Route::NotFound => html! {<PageNotFound/>},
            Route::Imprint => html! {<Imprint/>},
        }
    }
}

struct App;

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <BrowserRouter>
            <header class="p-3"><div class="container">
                <div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
                    <a class="d-flex align-items-center mb-2 mb-lg-0 text-white text-decoration-none" href="/">
                        <img class="bi me-2" width="40" height="32" role="img" aria-label="Schmetterling" src="favicon.svg" alt="Schmetterling"/>
                    </a>
                    <span>{"Schmetterling"}</span>
                </div>
            </div></header>
            <main>
                <Switch<Route> render={Switch::render(Route::switch)} />
            </main>
            <footer class="footer mt-auto py-3 bg-light"><div class="container">
                <span class="text-muted">{"Copyright (c) 2022 ProphetLamb"}</span>
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                <Link<Route> to={Route::Imprint}>{"Imprint"}</Link<Route>>
            </div></footer>
        </BrowserRouter>
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();

    yew::start_app::<App>();
}
