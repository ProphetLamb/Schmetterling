use closure::closure;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::edt::{MarkupEdit, Presentation};
use crate::action;
use crate::data;
use crate::id;
use crate::markup::*;

pub type TitleChange = Callback<(id::Card, String, Event)>;
pub type ContentChange = Callback<(id::Card, Markup, Event)>;
pub type ModeChange = Callback<(id::Card, Presentation, Event)>;
pub type ClickEvent = Callback<(id::Card, MouseEvent)>;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id: id::Card,
    pub title: String,
    pub content: Markup,
    #[prop_or_default]
    pub on_title_change: TitleChange,
    #[prop_or_default]
    pub on_content_change: ContentChange,
    #[prop_or_default]
    pub on_click: ClickEvent,
    #[prop_or_default]
    pub on_double_click: ClickEvent,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let id = props.id;
    let state = use_reducer_eq(|| data::Card {
        mode: Presentation::View,
    });

    let content = &props.on_content_change;
    let content = Callback::from(closure!(clone content, |(markup, e): (Markup, Event)| {
        content.emit((id, markup, e));
    }));

    let title = &props.on_title_change;
    let title = closure!(clone title, |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().expect("Target must be HtmlInputElement.");
        title.emit((id, input.value(), e));
    });

    let click = &props.on_click;
    let click = closure!(clone click, |e| click.emit((id, e)));

    let double_click = &props.on_double_click;
    let double_click = closure!(clone state, clone double_click, |e: MouseEvent| {
        state.dispatch(action::Card::DoubleClick(e.clone()));
        double_click.emit((id, e));
    });

    let on_blur = closure!(clone state, |e: FocusEvent| {
        state.dispatch(action::Card::Blur(e));
    });

    html! {
    <div class="Card" id={format!("card-{}", id.value)} onclick={click} ondblclick={double_click} onblur={on_blur}>
        <div class="card-header">
        if state.mode == Presentation::View {
            <span class="card-title">{props.title.clone()}</span>
        } else {
            <input class="form-control" type="text" value={props.title.clone()} onchange={title} />
        }
        </div>
        <MarkupEdit edit_classes={classes!("card-body", "form-control")} view_classes={classes!("card-body")} mode={state.mode} value={props.content.clone()} on_change={content} />
    </div>
    }
}
