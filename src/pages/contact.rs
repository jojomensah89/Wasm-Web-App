use crate::router::Route;
use yew::{function_component, html, Callback, Html,classes};
use yew_router::hooks::use_navigator;

#[function_component(Contact)]
pub fn contact() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Profile));

    html!(
        <div class={classes!("container")}>
            <h1>{"Contact Page"}</h1>
            <button {onclick}>{"Go to login"}</button>
        </div>
    )
}
