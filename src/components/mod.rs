use edt::*;
use yew::{html, Component, Context, Html};

pub mod edt;

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div><div class="container">
                <div class="row" style="height: 100px"> </div>
               <div class="row" style="justify-content:center">
                    <p>
                        {"ProphetLamb"} <span style="color: #feffff ">{" @ "} </span> {"2021"}
                    </p>
               </div>
            </div></div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }
}

pub struct Header {}

impl Component for Header {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="container">
                <div class="row">
                    <ul class="nav col">
                    </ul>
                    <div class="col" style="text-align:center">
                        <h4 class="nav-link font-weight-bold"> {"Todo app in Yew"} </h4>
                    </div>
                    <ul class="nav col" style="visibility: hidden">
                    </ul>
                </div>
            </div>
        }
    }
}

pub struct Home {}

impl Component for Home {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div>
                <Edt></Edt>
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        true
    }
}
