#![recursion_limit = "1024"]

pub mod components;
pub mod data;
pub mod pages;
pub mod route;
use yew::prelude::*;

use crate::{components::*, pages::home::Home};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Main;

impl Component for Main {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <>
            <Navbar/>
            <Home/>
            <Footer/>
        </>
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<Main>();
}
