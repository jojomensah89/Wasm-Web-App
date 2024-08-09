use crate::router::Route::Contact;
use yew::{function_component, html, Callback, Html};
use yew_router::hooks::use_navigator;

#[function_component(Profile)]
pub fn profile() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Contact));

    html!(
        <div>
            <h1>{"Profile Page"}</h1>
            <button {onclick}>{"Go to contact"}</button>
        </div>
    )
}
