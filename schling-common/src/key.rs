use std::{collections::HashMap, fmt::Display};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, KeyboardEvent};
use yew::{Callback, TargetCast};

#[derive(Clone, Default)]
pub struct KeyStack {
    pressed: Vec<String>,
}

impl Display for KeyStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pressed.join(" "))
    }
}

impl KeyStack {
    pub fn key_down(&mut self, key: &str) -> Option<usize> {
        match self.find(key) {
            Ok(_) => None,
            Err(pos) => {
                self.pressed.insert(pos, key.to_string());
                Some(pos)
            }
        }
    }

    pub fn key_up(&mut self, key: &str) -> Option<usize> {
        match self.find(key) {
            Ok(pos) => {
                self.pressed.remove(pos);
                Some(pos)
            }
            Err(_) => None,
        }
    }

    /// see binary_search
    pub fn find(&self, key: &str) -> Result<usize, usize> {
        self.pressed.binary_search_by(|item| item.as_str().cmp(key))
    }
}

#[derive(Default, PartialEq)]
pub struct KeyShortcuts {
    patterns: HashMap<String, Callback<KeyboardEvent>>,
}

impl KeyShortcuts {
    pub fn register<F: 'static + Fn(KeyboardEvent) + Sized>(&mut self, pattern: &str, cb: F) {
        self.patterns
            .insert(pattern.to_string(), Callback::from(cb));
    }

    pub fn register_target<F, T>(&mut self, pattern: &str, cb: F)
    where
        F: 'static + Fn(T, KeyboardEvent) + Sized,
        T: AsRef<EventTarget> + JsCast,
    {
        self.register(pattern, move |e: KeyboardEvent| {
            match e.target_dyn_into::<T>() {
                Some(t) => cb(t, e),
                None => {}
            };
        });
    }

    pub fn handle_keypress(&self, keys: &KeyStack, event: KeyboardEvent) {
        if let Some(callback) = self.patterns.get(&keys.to_string()) {
            callback.emit(event)
        }
    }
}
