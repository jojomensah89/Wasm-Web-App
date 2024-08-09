use crate::pages::contact::Contact;
use crate::pages::login::Login;
use crate::pages::profile::Profile;
use yew::{html, Html};
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/profile")]
    Profile,
    #[at("/contact")]
    Contact,
    #[at("/")]
    Login,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Profile => html! {<Profile/>},
        Route::Contact => html! {<Contact/>},
        _ => html! {<Login/>},
    }
}
