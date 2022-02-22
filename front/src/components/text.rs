use closure::closure;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use schling_common::markup::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Presentation {
    View,
    Edit,
}

macro_rules! callback_event {
    ($props:tt, $event:ty, $handler:tt) => {
        {
            let $handler = &$props.$handler;
            Callback::from(closure!(clone $handler, |e: $event| $handler.emit(e)))
        }
    };
}

#[derive(Properties, PartialEq, Debug)]
pub struct TextProps {
    #[prop_or(Presentation::View)]
    pub mode: Presentation,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub view_classes: Classes,
    #[prop_or_default]
    pub edit_classes: Classes,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_double_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_input: Callback<InputEvent>,
    #[prop_or_default]
    pub on_change: Callback<(String, Event)>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
}

enum TextAction {
    Invalidate(String),
}

#[derive(PartialEq, Debug)]
struct TextState {
    value: String,
}

impl Reducible for TextState {
    type Action = TextAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TextAction::Invalidate(value) => Self { value }.into(),
        }
    }
}

fn text_change(state: UseReducerHandle<TextState>, props: &TextProps) -> Callback<Event> {
    let onchange = props.on_change.clone();
    Callback::from(move |e: Event| {
        let area: HtmlInputElement = e.target_dyn_into().expect("Must be a textarea.");
        let value = area.value();

        state.dispatch(TextAction::Invalidate(value.clone()));
        onchange.emit((value, e))
    })
}

#[function_component(TextEdit)]
pub fn text(props: &TextProps) -> Html {
    let state = use_reducer_eq(|| TextState {
        value: props.value.clone(),
    });
    let onclick = callback_event!(props, MouseEvent, on_click);
    let ondblclick = callback_event!(props, MouseEvent, on_double_click);
    let oninput = callback_event!(props, InputEvent, on_input);
    let onblur = callback_event!(props, FocusEvent, on_blur);
    match props.mode {
        Presentation::View => html! {
        <span class={props.view_classes.clone()} {onclick} {ondblclick} {oninput} {onblur}>
            {props.value.clone()}
        </span>
        },
        Presentation::Edit => {
            let onchange = text_change(state, props);
            html! {
            <input class={props.edit_classes.clone()} type="text" value={props.value.clone()} {onclick} {ondblclick} {oninput} {onchange} {onblur} />
            }
        }
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct MarkupProps {
    #[prop_or(Presentation::View)]
    pub mode: Presentation,
    #[prop_or_default]
    pub value: Markup,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub view_classes: Classes,
    #[prop_or_default]
    pub edit_classes: Classes,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_double_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_input: Callback<InputEvent>,
    #[prop_or_default]
    pub on_change: Callback<(Markup, Event)>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
}

enum MarkupAction {
    Invalidate(Markup),
}

#[derive(PartialEq, Debug)]
struct MarkupState {
    dom: Html,
    value: Markup,
}

impl MarkupState {
    pub fn with_value(&self, value: Markup) -> Self {
        Self {
            dom: value.to_dom(),
            value,
        }
    }
}

impl Reducible for MarkupState {
    type Action = MarkupAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MarkupAction::Invalidate(value) => self.with_value(value).into(),
        }
    }
}

fn markup_change(state: UseReducerHandle<MarkupState>, props: &MarkupProps) -> Callback<Event> {
    let onchange = props.on_change.clone();
    let lang = props.value.lang;
    Callback::from(closure!(clone lang, |e: Event| {
        let area: HtmlTextAreaElement = e.target_dyn_into().expect("Must be a textarea.");
        let markup = lang.with_text(area.value());

        state.dispatch(MarkupAction::Invalidate(markup.clone()));
        onchange.emit((markup, e))
    }))
}

#[function_component(MarkupEdit)]
pub fn markup(props: &MarkupProps) -> Html {
    let state = use_reducer_eq(|| MarkupState {
        dom: props.value.to_dom(),
        value: props.value.clone(),
    });
    let onclick = callback_event!(props, MouseEvent, on_click);
    let ondblclick = callback_event!(props, MouseEvent, on_double_click);
    let oninput = callback_event!(props, InputEvent, on_input);
    let onblur = callback_event!(props, FocusEvent, on_blur);
    match props.mode {
        Presentation::View => {
            html! {
            <div class={props.view_classes.clone()} id={props.id.clone()} {onclick} {ondblclick} {oninput} {onblur}>
                {state.dom.clone()}
            </div>
            }
        }
        Presentation::Edit => {
            let onchange = markup_change(state, props);
            html! {
            <textarea class={props.edit_classes.clone()} value={props.value.text.clone()} id={props.id.clone()} {onclick} {ondblclick} {oninput} {onchange} {onblur} />
            }
        }
    }
}
