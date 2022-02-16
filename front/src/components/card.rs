use std::{collections::HashSet, rc::Rc};

use crate::markup::*;
use chrono::*;
use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Presentation {
    View,
    Edit,
}

pub enum EdtAction {
    Invalidate,
    Show(Presentation),
}

#[derive(PartialEq, Debug)]
pub struct EdtState {
    mode: Presentation,
    value: Markup,
    dom: Html,
    classes: Classes,
}

impl Reducible for EdtState {
    type Action = EdtAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            EdtAction::Invalidate => {
                self.dom = self.value.to_dom();
            }
            EdtAction::Show(mode) => {
                self.mode = mode;
            }
        }
        self
    }
}

#[function_component(Edt)]
pub fn edt() -> Html {
    let state = use_reducer_eq(|| {
        let value = Markup::md_str("");
        EdtState {
            mode: Presentation::View,
            value,
            dom: value.to_dom(),
            classes: "".into(),
        }
    });

    match state.mode {
        Presentation::View => state.dom,
        Presentation::Edit => {
            html! {
                <textarea class={state.classes} value={state.value.text}/>
            }
        }
    }
}
