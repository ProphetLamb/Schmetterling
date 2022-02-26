use closure::closure;
use schling_common::{id, markup::Markup};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::{
    components::code_area,
    data::{self, Head},
    dyn_into, query_parents,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: id::Sec,
}

const SECTION_PREFIX: &str = "section-";

macro_rules! query_parent_section {
    ($target:ident) => {
        query_parents!($target, |parent| match parent.dyn_into::<HtmlElement>() {
            Ok(parent) => {
                if parent.id().starts_with(SECTION_PREFIX) {
                    Ok(parent)
                } else {
                    Err(parent.dyn_into::<Node>().unwrap())
                }
            }
            Err(parent) => Err(parent.dyn_into::<Node>().unwrap()),
        })
    };
}

fn section_id(id: id::Sec) -> String {
    format!("{}{}", SECTION_PREFIX, id.value)
}

fn section_content_id(id: id::Sec) -> String {
    format!("{}content-{}", SECTION_PREFIX, id.value)
}

#[derive(PartialEq)]
struct State {
    edit: bool,
    section: data::Sec,
}

impl State {
    fn id(&self) -> id::Sec {
        self.section.head.id
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::Edit => {
                if self.edit {
                    self
                } else {
                    Self {
                        edit: true,
                        section: self.section.clone(),
                    }
                    .into()
                }
            }
            Action::View => {
                if self.edit {
                    Self {
                        edit: false,
                        section: self.section.clone(),
                    }
                    .into()
                } else {
                    self
                }
            }
            Action::UpdTitle(title) => Self {
                edit: self.edit,
                section: self.id().update(self.section.own_with_title(title)),
            }
            .into(),
            Action::UpdContent(content) => Self {
                edit: self.edit,
                section: self.id().update(self.section.own_with_content(content)),
            }
            .into(),
            Action::ViewKeyPress(e) => {
                let key = e.key();
                let key = key.as_str();
                match key {
                    " " | "Enter" => {
                        // Space | Enter
                        Self {
                            edit: true,
                            section: self.section.clone(),
                        }
                        .into()
                    }
                    _ => self,
                }
            }
        }
    }
}

enum Action {
    Edit,
    View,
    UpdTitle(String),
    UpdContent(Markup),
    ViewKeyPress(KeyboardEvent),
}

fn new_section(id: id::Sec) -> data::Sec {
    data::SecHead::new(id, id.value, format!("Section {}", id.value)).body(Markup::md_str(""))
}

#[function_component(Section)]
pub fn section(props: &Props) -> Html {
    let Props { id } = props.clone();
    let state = use_reducer_eq(|| State {
        edit: false,
        section: id.load().unwrap_or_else(|| new_section(id)),
    });
    let data::Sec { head, content } = state.section.clone();

    let title = head.title;
    if state.edit {
        let content = content.text;
        let upd_title = Callback::from(closure!(clone state, |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
            let value = target.value();
            state.dispatch(Action::UpdTitle(value));}
        }));
        let upd_content = Callback::from(closure!(clone state, |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlTextAreaElement>(){
            let value = Markup::md(target.value());
            state.dispatch(Action::UpdContent(value));}
        }));
        let view = Callback::from(closure!(clone state, |_| state.dispatch(Action::View)));
        html! {
        <section id={section_id(id)} class="box">
            <div class="level">
                <input class="input" type="text" placeholder="Section title" value={title} onchange={upd_title}/>
            </div>
            <code_area::CodeArea id={section_content_id(id)} class="textarea section-content" value={content} placeholder="Section content" onchange={upd_content}/>
            <div class="level mt-3">
                <button class="button is-primary is-rounded" onclick={view}>{"Update"}</button>
            </div>
        </section>
        }
    } else {
        let content = content.to_dom();
        let edit = Callback::from(closure!(clone state, |_| state.dispatch(Action::Edit)));
        let keypress = Callback::from(move |e| state.dispatch(Action::ViewKeyPress(e)));
        html! {
        <section class="box" ondblclick={edit} tabindex=0 onkeypress={keypress}>
            <div class="level">
                <span class="level-item title">{title}</span>
            </div>
            {content}
        </section>
        }
    }
}
