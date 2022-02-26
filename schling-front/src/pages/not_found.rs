use yew::prelude::*;

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
    <div class="justify-center">
        <h1>{"404"}</h1>
        <p>{"This is not the page you are looking for."}</p>
    </div>
    }
}
