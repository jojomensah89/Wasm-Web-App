use crate::router::{switch, Route};
use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
        <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}
