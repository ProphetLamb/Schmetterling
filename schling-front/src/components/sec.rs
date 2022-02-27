use closure::closure;
use schling_common::{id, markup::Markup};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::code_area::CodeArea;
use crate::components::code_area::CodeStyle;
use crate::data::{self, Head};

use super::code_area::Text;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: id::Sec,
}

const SECTION_PREFIX: &str = "section-";

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
        let content = Text {
            lines: content.text,
            style: CodeStyle::default(),
        };
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
            <CodeArea value={content} class="textarea section-content" placeholder="Section content"/>
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
