use std::collections::{hash_map::Entry::*, HashMap};

use crate::components::{doc, proj, text::Presentation};
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
    let decl = if let Some(doc) = proj_get(id.proj).children.get(&id) {
        doc.clone()
    } else {
        doc::Decl::with_id(id)
    };
    doc::State {
        id,
        decl,
        decl_mode: Presentation::View,
        children,
    }
}

pub fn doc_upd_decl(decl: doc::Decl) -> doc::Decl {
    let mut proj = proj_get(decl.id.proj);
    match proj.children.entry(decl.id) {
        Occupied(mut entry) => {
            entry.insert(decl.clone());
        }
        Vacant(entry) => {
            entry.insert(decl.clone());
        }
    }
    proj_upd(proj);
    decl
}
