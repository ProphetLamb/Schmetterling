use closure::closure;
use schling_common::id;
use yew::prelude::*;

use super::sec::Section;
use crate::data::{self, Head, SecHead};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: id::Doc,
}

#[derive(PartialEq)]
struct State {
    document: data::Doc,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::Push => {
                let mut sections = self.document.content.clone();
                let id = self.document.head.id;
                let sec_id = sections.len() as u32;
                sections.push(SecHead::new(
                    id::Sec::new(sec_id, id),
                    sec_id,
                    String::default(),
                ));
                Self {
                    document: id.update(self.document.own_with_content(sections)),
                }
                .into()
            }
        }
    }
}

enum Action {
    Push,
}

fn new_document(id: id::Doc) -> data::Doc {
    data::DocHead::new(id, id.value, format!("Document {}", id.value)).body(Vec::default())
}

#[function_component(SecList)]
pub fn sec_list(props: &Props) -> Html {
    let Props { id } = props.clone();
    let state = use_reducer_eq(|| State {
        document: id.load().unwrap_or_else(|| new_document(id)),
    });

    let push = Callback::from(closure!(clone state, |_| {
        state.dispatch(Action::Push);
    }));
    html! {
    <>
    <div>
        {render_section(&state.document)}
    </div>
    <div class="level mt-3">
        <button class="level-item button is-rounded is-link is-outlined" onclick={push}>
            <i class="fa fa-plus"></i>
        </button>
    </div>
    </>
     }
}

fn render_section(document: &data::Doc) -> Html {
    html! {
    { for document.content.iter().cloned().map(move |sec| {
        html!(<Section id={sec.id} />)
    }) }
    }
}
