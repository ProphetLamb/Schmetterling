use closure::closure;
use multimap::MultiMap;
use std::{
    collections::HashSet,
    fmt::Display,
    sync::atomic::{AtomicUsize, Ordering},
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, KeyboardEvent, Node};
use yew::prelude::*;
use yew_agent::{Agent, AgentLink, Bridge, Bridged, Context, HandlerId};

use crate::invoke::Invoke;

/// Inserts the following snippet
/// ```rust
/// js_obj.and_then(|obj| obj.dyn_into::<target>.ok())
/// ```
#[macro_export]
macro_rules! dyn_into {
    ($js_obj:expr, $target:ty) => {{
        use wasm_bindgen::JsCast;
        $js_obj.and_then(|obj| obj.dyn_into::<$target>().ok())
    }};
}

/// # Returns
/// Returns Some() if the focus switches to an element in the object tree of the div, by locating a specific id, otherwise
/// None the focus switches outside of the current object tree.
pub fn blur_inside(e: FocusEvent, tag: &str) -> Option<HtmlDivElement> {
    let related = dyn_into!(e.related_target(), Node);
    query_parents(related, |node| match node.dyn_into::<HtmlDivElement>() {
        Ok(div) => {
            if div.id() == tag {
                Ok(div)
            } else {
                Err(div.dyn_into::<Node>().expect("div must also be a node."))
            }
        }
        Err(node) => Err(node),
    })
}

/// Traverses the parentage of a specific `target` Node:
///
/// # Returns
/// Returns Some(target) if the `selector` yields Ok(), otherwise
/// returns None all parents of the node in the document yield Err().
pub fn query_parents<N: JsCast, F: Fn(Node) -> Result<N, Node>>(
    target: Option<Node>,
    selector: F,
) -> Option<N> {
    let mut parent = target;

    while let Some(target) = parent {
        match selector(target) {
            Ok(result) => return Some(result),
            Err(target) => {
                parent = target.parent_node();
            }
        }
    }

    return None;
}

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

    pub fn clear(&mut self) {
        self.pressed.clear()
    }

    /// see binary_search
    pub fn find(&self, key: &str) -> Result<usize, usize> {
        self.pressed.binary_search_by(|item| item.as_str().cmp(key))
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

pub enum Action {
    Register(Shortcut),
    Unregister(Shortcut),
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
    Blur(FocusEvent),
}

#[derive(PartialEq)]
pub struct Shortcut {
    pub pattern: String,
    pub can_execute: Invoke<KeyboardEvent, bool>,
    pub execute: Callback<KeyboardEvent>,
}

impl Shortcut {
    pub fn new(
        pattern: &str,
        can_execute: Invoke<KeyboardEvent, bool>,
        execute: Callback<KeyboardEvent>,
    ) -> Self {
        Self {
            pattern: pattern.to_string(),
            can_execute,
            execute,
        }
    }

    pub fn new_always(pattern: &str, execute: Callback<KeyboardEvent>) -> Self {
        Self {
            pattern: pattern.to_string(),
            can_execute: Invoke::from(|_| true),
            execute,
        }
    }
}

static CTX_ID: AtomicUsize = AtomicUsize::new(0);
static CTX_TAG: &str = "key-context-";

pub struct KeyContext {
    id: usize,
    hotkeys: MultiMap<String, Shortcut>,
    keys: KeyStack,
    _register: Box<dyn Bridge<KeyRegistrar>>,
}

impl Component for KeyContext {
    type Message = Action;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            id: CTX_ID.fetch_add(1, Ordering::SeqCst),
            hotkeys: MultiMap::default(),
            keys: KeyStack::default(),
            _register: KeyRegistrar::bridge(ctx.link().callback(Action::Register)),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let Props { class, children } = ctx.props().clone();
        let link = ctx.link();

        let id = format!("{}{}", CTX_TAG, self.id);
        let onkeydown = link.callback(Action::KeyDown);
        let onkeyup = link.callback(Action::KeyUp);
        let onblur = link.callback(Action::Blur);
        html! {
            <div {id} {class} {onkeydown} {onkeyup} {onblur}>
                {children}
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Action::Register(shortcut) => {
                self.hotkeys.insert(shortcut.pattern.clone(), shortcut);
                false
            }
            Action::Unregister(shortcut) => {
                if let Some(shortcuts) = self.hotkeys.get_vec_mut(&shortcut.pattern) {
                    if let Some(idx) = shortcuts.iter().position(|s| s == &shortcut) {
                        shortcuts.remove(idx);
                    }
                }
                false
            }
            Action::KeyDown(key) => {
                let pattern = key.key();
                self.keys.key_down(&pattern);
                if let Some(shortcuts) = self.hotkeys.get_vec(&pattern) {
                    if let Some(shortcut) =
                        shortcuts.iter().find(|s| s.can_execute.emit(key.clone()))
                    {
                        shortcut.execute.emit(key);
                    }
                }
                false
            }
            Action::KeyUp(key) => {
                self.keys.key_up(&key.key());
                false
            }
            Action::Blur(focus) => {
                let id = format!("{}{}", CTX_TAG, self.id);
                // Focus moved outside of context
                if blur_inside(focus, &id).is_none() {
                    self.keys.clear();
                }
                false
            }
        }
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>) -> bool {
        false
    }
}

pub struct KeyRegistrar {
    link: AgentLink<KeyRegistrar>,
    subs: HashSet<HandlerId>,
}

impl Agent for KeyRegistrar {
    type Reach = Context<Self>;

    type Message = ();

    type Input = Shortcut;

    type Output = Shortcut;

    fn create(link: yew_agent::AgentLink<Self>) -> Self {
        Self {
            link,
            subs: HashSet::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: yew_agent::HandlerId) {
        if self.subs.contains(&id) {
            self.link.respond(id, msg)
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subs.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subs.remove(&id);
    }
}
