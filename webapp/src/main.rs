use dioxus::prelude::*;
mod components;
mod layouts;
mod pages;

use layouts::with_navbar_footer::WithNavbarFooter;
use pages::chat::ChatPage;
use pages::login::LoginPage;
use pages::user_profile::UserProfilePage;

fn main() {
    launch(app);
}

#[rustfmt::skip]
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/login")]
    LoginPage {},
    #[layout(WithNavbarFooter)]
    #[route("/")]
    ChatPage {},
    #[nest("/u")]
        #[route("/:id")]
        UserProfilePage { id: i32},
}

pub fn app() -> Element {
    rsx! {
        Router::<Route> { }
    }
}
