use dioxus::prelude::*;

#[component]
pub fn message_list() -> Element {
    rsx! {
        nav {
            style: "
            padding: 0.7rem 1rem 0.7rem 2rem;
            margin: 0.5rem 0.5rem 0.5rem 0.5rem;
            border-radius: 0.5rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            ",
            p
            {
                style: "
                font-size: 1.5rem;
                font-weight: 600;
                ",
                "ChatApp"
            }
            ul {
                style: "
                font-weight: 500;
                ",
                li {
                "Login"
                }
            }
        }
        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
    }
}
