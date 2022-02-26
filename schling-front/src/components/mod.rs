pub mod code_area;
pub mod sec;
pub mod sec_list;

use schling_common::markup::Markup;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::prelude::*;

#[macro_export]
macro_rules! pass_thru {
    ($props:ident.$name:ident, $($closure:tt)*) => {
        {
            let $name = $props.$name.clone();
            Callback::from(move $($closure)*)
        }
    };
    ($reducer:ident, |$event:ident| $($action:tt)*) => {
        {
            let $reducer = $reducer.clone();
            Callback::from(move |$event| $reducer.dispatch($($action)*))
        }
    };
    ($reducer:ident, |$event:ident: $tevent:ty| $($action:tt)*) => {
        {
            let $reducer = $reducer.clone();
            Callback::from(move |$event: $tevent| $reducer.dispatch($($action)*))
        }
    };
}

#[macro_export]
macro_rules! dyn_into {
    ($js_obj:expr, $target:ty) => {{
        use wasm_bindgen::JsCast;
        $js_obj.and_then(|obj| obj.dyn_into::<$target>().ok())
    }};
}

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

#[macro_export]
macro_rules! query_parents {
    ($target:ident, $($selector:tt)*) => {
        query_parents(dyn_into!($target, Node), $($selector)*)
    };
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
    <nav class="navbar" role="navigation" aria-label="main navigation">
        <div class="navbar-brand">
            <a class="navbar-item" href="">
                <img src="assets/favicon.svg" alt="Schmetterling" height="28"/>
            </a>
            // burger menu for mobile devices
            <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="main_navigation">
                <span aria-hidden="true" />
                <span aria-hidden="true" />
                <span aria-hidden="true" />
            </a>
        </div>
        <div id="main_navigation" class="navbar-menu">
            <div class="navbar-start">
                <a class="navbar-item">{"Home"}</a>
            </div>
            <div class="navbar_end">
            </div>
        </div>
    </nav>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    let about = include_str!("../../ABOUT.md");
    let about = Markup::md_str(about);
    html! {
    <footer class="footer">
        <div class="content has-text-centered">
            {about.to_dom()}
        </div>
    </footer>
    }
}
