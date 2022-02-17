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
            <nav class="navbar navbar-expand navbar-dark bg-dark">
            <div class="container-fluid">
                <a class="navbar-brand" href="/">
                    <img class="bi me-2" width="40" height="32" role="img" src="assets/favicon.svg" alt="Schmetterling"/>
                </a>
                <div class="collapse navbar-collapse">
                <ul class="navbar-nav me-auto">
                    <li class="nav-item"><Link<Route> classes="nav-link active" to={Route::Home}>{"Home"}</Link<Route>></li>
                </ul>
                </div>
            </div>
            </nav>
            <main class="flex-shrink-0" style="min-height: 30rem;">
                <div class="container">
                <Switch<Route> render={Switch::render(Route::switch)} />
                </div>
            </main>
            <footer class="footer mt-auto py-3">
            <div class="d-flex justify-content-right flex-row flex-wrap">
                <a class="text-muted nav-link" href="https://github.com/ProphetLamb/Schmetterling">{"Copyright (c) 2022 ProphetLamb"}</a>
                <Link<Route> classes="text-muted nav-link" to={Route::Home}>{"Home"}</Link<Route>>
                <Link<Route> classes="text-muted nav-link" to={Route::Imprint}>{"Imprint"}</Link<Route>>
            </div>
            </footer>
        </BrowserRouter>
        }
    }
}

#[macro_export]
macro_rules! web_log {
    ($($arg:tt)*) => {
        web_sys::console::log_1(
                &format!($($arg)*).into()
            );

    };
}
#[macro_export]
macro_rules! web_warn {
    ($($arg:tt)*) => {
        web_sys::console::warn_1(
                &format!($($arg)*).into()
            );

    };
}
#[macro_export]
macro_rules! web_error {
    ($($arg:tt)*) => {
        web_sys::console::error_1(
                &format!($($arg)*).into()
            );

    };
}

fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}
