use closure::closure;
use yew::{prelude::*, props};

use super::card;

pub enum Action {
    Add(i64),
    Edit(i64),
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id: i64,
}

pub struct Document {
    title: String,
    children: Vec<i64>,
}

impl Component for Document {
    type Message = Action;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            title: String::default(),
            children: Vec::with_capacity(32),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Action::Add(id) => {
                self.children.push(id);
                true
            }
            Action::Edit(id) => true,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let id = self.children.len() as i64;
        let add = link.callback(move |_| Action::Add(id));

        html! {
        <section class="Document">
            <div>
                <h1>{self.title.clone()}</h1>
            </div>
            <div class="container">
            { for self.children.iter().map(|id| html_nested!{
                <card::Card id={*id} />
            }) }
            </div>
            <div class="d-flex m-3">
                <button class="btn btn-circle" onclick={add}>
                    <i class="fa fa-solid fa-plus fa-xl"></i>
                </button>
            </div>
        </section>
        }
    }
}
