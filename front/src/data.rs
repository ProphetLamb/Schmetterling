use std::collections::HashMap;
use std::{collections::hash_map::Entry::*, rc::Rc};

use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlDivElement, HtmlElement, Node};
use yew::prelude::*;

use crate::{action, components::edt::Presentation, id, markup::Markup};

pub fn proj_get(id: id::Proj) -> Proj {
    crate::get_or_create(&format!("{}", id), || Proj::with_id(id))
}

pub fn proj_upd(state: Proj) -> Proj {
    crate::set(&format!("{}", state.id), state)
}

pub fn doc_upd_children(state: Doc) -> Doc {
    crate::set(&format!("{}", state.id), state.children.clone());
    state
}

pub fn doc_get(id: id::Doc) -> Doc {
    let children = crate::get_or_create(&format!("{}", id), HashMap::default);
    let title = if let Some(doc) = proj_get(id.proj).children.get(&id) {
        doc.title.clone()
    } else {
        format!("Document {}", id.value)
    };
    Doc {
        id,
        title,
        title_mode: Presentation::View,
        children,
    }
}

pub fn doc_upd_title(state: Doc) -> Doc {
    let mut proj = proj_get(state.id.proj);
    match proj.children.entry(state.id) {
        Occupied(mut entry) => {
            entry.get_mut().title = state.title.clone();
        }
        Vacant(entry) => {
            entry.insert(ProjDoc {
                id: state.id,
                title: state.title.clone(),
                summary: Markup::md_str(""),
                order: state.id.value,
            });
        }
    }
    proj_upd(proj);
    state
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Proj {
    pub id: id::Proj,
    pub children: HashMap<id::Doc, ProjDoc>,
}

impl Proj {
    fn with_id(id: id::Proj) -> Self {
        Self {
            id,
            children: HashMap::default(),
        }
    }

    pub fn doc_next(&self) -> id::Doc {
        id::Doc {
            value: self.children.len() as u64,
            proj: self.id,
        }
    }
}

impl Reducible for Proj {
    type Action = action::Proj;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            action::Proj::Add(id) => {
                let mut state = (*self).clone();
                state.children.insert(id, ProjDoc::with_id(id));
                proj_upd(state).into()
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct ProjDoc {
    pub id: id::Doc,
    pub title: String,
    pub summary: Markup,
    pub order: u64,
}

impl Ord for ProjDoc {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for ProjDoc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl ProjDoc {
    fn with_id(id: id::Doc) -> Self {
        Self {
            id,
            title: format!("Document {}", id.value),
            summary: Markup::md_str(""),
            order: id.value,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Doc {
    pub id: id::Doc,
    pub title: String,
    pub title_mode: Presentation,
    pub children: HashMap<id::Card, DocCard>,
}

impl Doc {
    pub fn card_next(&self) -> id::Card {
        id::Card {
            value: self.children.len() as u64,
            doc: self.id,
        }
    }

    fn own_title_view(&self, title: String) -> Self {
        Self {
            id: self.id,
            title,
            children: self.children.clone(),
            title_mode: Presentation::View,
        }
    }

    fn own_children(&self, children: HashMap<id::Card, DocCard>) -> Self {
        Self {
            id: self.id,
            title: self.title.clone(),
            children,
            title_mode: Presentation::View,
        }
    }

    fn own_mode(&self, title_mode: Presentation) -> Self {
        Self {
            id: self.id,
            title: self.title.clone(),
            children: self.children.clone(),
            title_mode,
        }
    }
}

impl Reducible for Doc {
    type Action = action::Doc;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            action::Doc::Add(id) => {
                let mut state = (*self).clone();
                state.children.insert(id, DocCard::with_id(id));
                doc_upd_children(state).into()
            }
            action::Doc::CardTitle((id, title, _)) => {
                if matches!(self.children.get(&id), Some(card) if card.title != title) {
                    let mut children = self.children.clone();
                    if let Some(card) = children.get_mut(&id) {
                        card.title = title;
                    }
                    let state = self.own_children(children);
                    doc_upd_children(state).into()
                } else {
                    self
                }
            }
            action::Doc::CardContent((id, content, _)) => {
                if matches!(self.children.get(&id), Some(card) if card.content != content) {
                    let mut state = (*self).clone();
                    if let Some(card) = state.children.get_mut(&id) {
                        card.content = content;
                    }
                    doc_upd_children(state).into()
                } else {
                    self
                }
            }
            action::Doc::Title(title) => {
                if self.title != title {
                    let state = self.own_title_view(title);
                    doc_upd_title(state).into()
                } else {
                    self
                }
            }
            action::Doc::TitleMode(mode) => self.own_mode(mode).into(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct DocCard {
    pub id: id::Card,
    pub title: String,
    pub content: Markup,
    pub order: u64,
}

impl Ord for DocCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for DocCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl DocCard {
    fn with_id(id: id::Card) -> Self {
        Self {
            id,
            title: format!("Section {}", id.value),
            content: Markup::default(),
            order: id.value,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Card {
    pub mode: Presentation,
}

impl Card {
    fn with_mode_view(&self) -> Self {
        self.with_mode(Presentation::View)
    }

    fn with_mode(&self, mode: Presentation) -> Self {
        Self { mode }
    }
}

impl Reducible for Card {
    type Action = action::Card;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            action::Card::Mode(mode) => {
                if self.mode == mode {
                    self
                } else {
                    self.with_mode(mode).into()
                }
            }
            action::Card::DoubleClick(_) => {
                if self.mode == Presentation::View {
                    self.with_mode(Presentation::Edit).into()
                } else {
                    self
                }
            }
            action::Card::Blur(event) => {
                let target = event
                    .target_dyn_into::<HtmlElement>()
                    .expect_throw("Expected event target HtmlElement.");
                if let Some(card) = map_parent(target, |n| match n.dyn_into::<HtmlDivElement>() {
                    Ok(div) if div.class_list().contains("Card") => Ok(div),
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
