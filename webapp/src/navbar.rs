use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            ul {
                style: "
                list-style-type: none;
                margin: 0;
                padding: 0;
                overflow: hidden;
                background-color: #333;
                ",
                li {
                    style: "
                    float: left;
                    ",
                    p {
                        style: "
                        display: block;
                        color: white;
                        text-align: center;
                        padding: 0.5rem 0.5rem;
                        text-decoration: none;
                        ",
                        "ChatApp"
                    }
                }
            }
        }
        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
    }
}
