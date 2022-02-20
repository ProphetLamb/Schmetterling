use std::collections::{hash_map::Entry::*, HashMap};

use crate::components::{doc, edt::Presentation, proj, sec};
use crate::{id, markup::Markup};

pub fn proj_get(id: id::Proj) -> proj::State {
    crate::get_or_create(&format!("{}", id), || proj::State::with_id(id))
}

pub fn proj_upd(state: proj::State) -> proj::State {
    crate::set(&format!("{}", state.id), state)
}

pub fn doc_upd_children(state: doc::State) -> doc::State {
    crate::set(&format!("{}", state.id), state.children.clone());
    state
}

pub fn doc_get(id: id::Doc) -> doc::State {
    let children = crate::get_or_create(&format!("{}", id), HashMap::default);
    let title = if let Some(doc) = proj_get(id.proj).children.get(&id) {
        doc.title.clone()
    } else {
        format!("Document {}", id.value)
    };
    doc::State {
        id,
        title,
        title_mode: Presentation::View,
        children,
    }
}

pub fn doc_upd_title(state: doc::State) -> doc::State {
    let mut proj = proj_get(state.id.proj);
    match proj.children.entry(state.id) {
        Occupied(mut entry) => {
            entry.get_mut().title = state.title.clone();
        }
        Vacant(entry) => {
            entry.insert(proj::Doc {
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
