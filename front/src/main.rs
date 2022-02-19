#![recursion_limit = "1024"]

pub mod components;
pub mod id;
pub mod markup;

use gloo_console::{error, info};
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use serde::{Deserialize, Serialize};
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
            <main>
                <Switch<Route> render={Switch::render(Route::switch)} />
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

macro_rules! tname {
    ($type:ty) => {
        std::any::type_name::<$type>()
    };
}

/// Attempts to deserialize a key in the `LocalStorage`, if successful returns the value;
/// otherwise creates the key of the given type, if that fails panics;
/// otherwise attempts to get the created key, if that fails panics.
pub fn get_or_create<T, F>(key: &str, f: F) -> T
where
    T: Serialize + for<'de> Deserialize<'de>,
    F: FnOnce() -> T,
{
    info!(format!("get key {} of type {}", key, tname!(T)));
    match LocalStorage::get(key) {
        Ok(set) => set,
        Err(err) => {
            error!(format!(
                "key {} not found. Creating key instead.\nError: {}",
                key, err
            ));
            let value = f();
            set(key, value)
        }
    }
}

pub fn set<T>(key: &str, value: T) -> T
where
    T: ?Sized + Serialize + for<'de> Deserialize<'de>,
{
    info!(format!("set key {} of type {}", key, tname!(T)));
    LocalStorage::set(key, &value).unwrap_or_else(|e| panic_set(key, e));
    value
}

fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}

fn panic_set(key: &str, e: StorageError) -> ! {
    panic!("unable to create the key '{}'.\nError: {}", key, e)
}
fn panic_get(key: &str, e: StorageError) -> ! {
    panic!("unable to get the created key '{}'.\nError: {}", key, e)
}
fn panic_ser(e: serde_json::Error) -> ! {
    panic!("unable to serialize the value.\nError: {}", e)
}
