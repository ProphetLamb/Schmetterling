use std::rc::Rc;

use closure::closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlElement, HtmlInputElement, Node};
use yew::prelude::*;

use super::text::{MarkupEdit, Presentation};
use schling_common::{id, markup::*};

pub enum Action {
    Mode(Presentation),
    DoubleClick(MouseEvent),
    Blur(FocusEvent),
}

#[derive(PartialEq, Debug, Clone)]
pub struct State {
    pub mode: Presentation,
}

impl State {
    fn with_mode_view(&self) -> Self {
        self.with_mode(Presentation::View)
    }

    fn with_mode(&self, mode: Presentation) -> Self {
        Self { mode }
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Mode(mode) => {
                if self.mode == mode {
                    self
                } else {
                    self.with_mode(mode).into()
                }
            }
            Action::DoubleClick(_) => {
                if self.mode == Presentation::View {
                    self.with_mode(Presentation::Edit).into()
                } else {
                    self
                }
            }
            Action::Blur(event) => {
                let target = event
                    .target_dyn_into::<HtmlElement>()
                    .expect("Expected event target HtmlElement.");
                if let Some(card) = map_parent(target, |n| match n.dyn_into::<HtmlDivElement>() {
                    Ok(div) if div.class_list().contains("Section") => Ok(div),
                    Ok(div) => Err(div.into()),
                    Err(n) => Err(n.unchecked_into::<HtmlElement>()),
                }) {
                    if let Some(related) = event
                        .related_target()
                        .and_then(|et| et.dyn_into::<Node>().ok())
                    {
                        if card.contains(Some(&related)) {
                            return self;
                        }
                    }
                }
                let state = self.with_mode_view();
                state.into()
            }
        }
    }
}

fn map_parent<T: JsCast, F: Fn(HtmlElement) -> Result<T, HtmlElement>>(
    item: HtmlElement,
    select: F,
) -> Option<T> {
    let mut root = item;
    while let Some(child) = root
        .parent_node()
        .and_then(|node| node.dyn_into::<HtmlElement>().ok())
    {
        match select(child) {
            Ok(target) => return Some(target),
            Err(child) => root = child,
        }
    }
    None
}

pub type ChangeEvent<T> = Callback<(id::Sec, T, Event)>;
pub type ClickEvent = Callback<(id::Sec, MouseEvent)>;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id: id::Sec,
    pub title: String,
    pub content: Markup,
    #[prop_or_default]
    pub on_title_change: ChangeEvent<String>,
    #[prop_or_default]
    pub on_content_change: ChangeEvent<Markup>,
    #[prop_or_default]
    pub on_click: ClickEvent,
    #[prop_or_default]
    pub on_double_click: ClickEvent,
}

#[function_component(Section)]
pub fn sec(props: &Props) -> Html {
    let id = props.id;
    let state = use_reducer_eq(|| State {
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
        state.dispatch(Action::DoubleClick(e.clone()));
        double_click.emit((id, e));
    });

    let on_blur = closure!(clone state, |e: FocusEvent| {
        state.dispatch(Action::Blur(e));
    });

    html! {
    <div class="Section" id={format!("card-{}", id.value)} onclick={click} ondblclick={double_click} onblur={on_blur}>
        <div class="card-header">
        if state.mode == Presentation::View {
            <span class="card-title">{props.title.clone()}</span>
        } else {
            <input class="form-control" type="text" value={props.title.clone()} onchange={title} autofocus=true />
        }
        </div>
        <MarkupEdit edit_classes={classes!("card-body", "form-control")} view_classes={classes!("card-body")} mode={state.mode} value={props.content.clone()} on_change={content} />
    </div>
    }
}
