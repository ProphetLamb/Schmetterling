use schling_common::{id, markup::Markup};
use yew::prelude::*;

use crate::data;

type Cb<T> = Callback<(id::Sec, T)>;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    edit: bool,
    section: data::Sec,
    on_edit: Cb<MouseEvent>,
    on_submit: Cb<MouseEvent>,
}

macro_rules! pass {
    ($props:ident.$name:ident, $($closure:tt)*) => {
        {
            let $name = $props.$name.clone();
            Callback::from(move $($closure)*)
        }
    };
}

#[function_component(Section)]
pub fn section(props: &Props) -> Html {
    let id = props.section.head.id;
    let title = props.section.head.title.clone();
    if props.edit {
        let content = props.section.content.text.clone();
        let edit = pass!(props.on_edit, |e| on_edit.emit((id, e)));
        html! {
        <div ondblclick={edit}>
            <div>
                <input type="text" value={title}/>
            </div>
            <textarea value={content}/>
        </div>
        }
    } else {
        let content = props.section.content.to_dom();
        let submit = pass!(props.on_submit, |e| on_submit.emit((id, e)));
        html! {
        <div>
            <div>
                <span>{title}</span>
            </div>
            <div>{content}</div>
            <div>
                <button onclick={submit}>{"Submit"}</button>
            </div>
        </div>
        }
    }
}
