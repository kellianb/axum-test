use crate::components::navbar::Navbar;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn WithNavbarFooter() -> Element {
    rsx! {
        Navbar {}
        main {
            Outlet::<Route> { }
        },
    }
}
