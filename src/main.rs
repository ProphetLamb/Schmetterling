#![recursion_limit = "1024"]
use crate::components::*;

pub mod components;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct App;

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <Header/>
            <Imprint/>
            <Footer/>
            </>
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();

    yew::start_app::<App>();
}
