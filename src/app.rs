use crate::components::*;
use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
                <Header/>

                <Home/>

                <Footer/>
            </>
        }
    }
}
