use crate::router::Route;
use yew::{function_component, html, Callback, Html};
use yew_router::hooks::use_navigator;
use Route::Login;

#[function_component(Profile)]
pub fn profile() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Login));

    html!(
        <div>
            <h1>{"Profile Page"}</h1>
            <button {onclick}>{"Go to login"}</button>
        </div>
    )
}
