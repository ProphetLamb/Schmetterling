use closure::closure;
use std::rc::Rc;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::markup::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Presentation {
    View,
    Edit,
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    #[prop_or(Presentation::View)]
    pub mode: Presentation,
    #[prop_or_default]
    pub value: Markup,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_dblclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_input: Callback<InputEvent>,
    #[prop_or_default]
    pub on_change: Callback<(Markup, Event)>,
}

enum Action {
    Invalidate(Markup),
}

#[derive(PartialEq, Debug)]
struct State {
    dom: Html,
    value: Markup,
}

impl State {
    pub fn with_value(&self, value: Markup) -> Self {
        Self {
            dom: value.to_dom(),
            value,
        }
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Invalidate(value) => self.with_value(value).into(),
        }
    }
}

fn on_change(state: UseReducerHandle<State>, props: &Props) -> Callback<Event> {
    let onchange = props.on_change.clone();
    let lang = props.value.lang;
    Callback::from(closure!(clone lang, |e: Event| {
        let area: HtmlTextAreaElement = e.target_dyn_into().expect("Must be a textarea.");
        let markup = lang.with_text(area.value());

        state.dispatch(Action::Invalidate(markup.clone()));
        onchange.emit((markup, e))
    }))
}

#[function_component(Edt)]
pub fn edt(props: &Props) -> Html {
    let state = use_reducer_eq(|| State {
        dom: props.value.to_dom(),
        value: props.value.clone(),
    });

    macro_rules! callback {
        ($event:ty, $handler:tt) => {
            {
                let $handler = &props.$handler;
                Callback::from(closure!(clone $handler, |e: $event| $handler.emit(e)))
            }
        };
    }
    match props.mode {
        Presentation::View => state.dom.clone(),
        Presentation::Edit => {
            let onclick = callback!(MouseEvent, on_click);
            let ondblclick = callback!(MouseEvent, on_dblclick);
            let oninput = callback!(InputEvent, on_input);
            let onchange = on_change(state, props);
            html! {
            <textarea class={props.classes.clone()} value={props.value.text.clone()} {onclick} {ondblclick} {oninput} {onchange}/>
            }
        }
    }
}
