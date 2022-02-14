use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

pub fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <h1>{ "Home" }</h1> },
        AppRoute::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
