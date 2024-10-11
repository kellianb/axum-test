use dioxus::prelude::*;
mod components;
mod layouts;
mod pages;

use layouts::with_navbar_footer::WithNavbarFooter;
use pages::chat::ChatPage;
use pages::login::LoginPage;
use pages::register::RegisterPage;
use pages::user_profile::UserProfilePage;

#[rustfmt::skip]
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/login")]
    LoginPage {},
    #[route("/register")]
    RegisterPage {},
    #[layout(WithNavbarFooter)]
    #[route("/")]
    ChatPage {},
    #[nest("/u")]
        #[route("/:id")]
        UserProfilePage { id: i32},
}

fn main() {
    launch(app);
}

pub fn app() -> Element {
    rsx! {
        Router::<Route> { }
    }
}
