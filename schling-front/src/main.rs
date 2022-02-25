#![recursion_limit = "1024"]

pub mod components;
pub mod data;
pub mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct App;

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {}
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}
