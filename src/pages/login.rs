use crate::router::Route;
use yew::{function_component, html, Callback, Html};
use yew_router::hooks::use_navigator;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Profile));

    html!(
        <div>
            <h1>{"Login Page"}</h1>
            <button {onclick}>{"Go to profile"}</button>
        </div>
    )
}
