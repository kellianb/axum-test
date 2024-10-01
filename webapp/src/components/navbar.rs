use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            style: "
            padding: 1rem 0rem 1rem 0rem;
            margin: 0.5rem 2rem 0.5rem 2rem;
            border-radius: 0.5rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            ",
            a
            {
                style: "
                font-size: 1.5rem;
                font-weight: 600;
                text-decoration: none;
                color: black;
                ",
                href: "/",
                "ChatApp"
            }
            ul {
                style: "
                font-weight: 500;
                ",
                li {
                    a {
                        style: "
                        text-decoration: none;
                        color: black;
                        ",
                        href: "/login",
                        "Login"
                    }
                }
            }
        }
        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
    }
}
