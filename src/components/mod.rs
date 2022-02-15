pub mod edt;

use web_sys::Element;
use yew::prelude::*;

use crate::components::edt::*;
use crate::markup::*;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct RawHtmlProps {
    pub inner_html: String,
}

pub struct RawHtml {
    props: RawHtmlProps,
    node_ref: NodeRef,
}

impl Component for RawHtml {
    type Message = ();
    type Properties = RawHtmlProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().to_owned(),
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unreachable!()
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let new_props = ctx.props();
        if self.props.ne(new_props) {
            self.props = new_props.to_owned();
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.node_ref.clone()}/>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let el = self.node_ref.cast::<Element>().unwrap();
        el.set_inner_html(&self.props.inner_html);
    }
}

pub struct Home;

impl Component for Home {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <Edt></Edt>
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
        Markup::Markdown(README.to_string()).to_html()
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
