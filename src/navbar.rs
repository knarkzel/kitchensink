use crate::*;
use dioxus_router::{use_router, Link};

pub fn NavBar(cx: Scope) -> Element {
    // Hooks
    let router = use_router(cx);
    
    cx.render(rsx! {
        nav {
            class: "navbar has-background-light",
            div {
                class: "navbar-start",
                Link { class: "navbar-item", to: "/", "Home" },
                Link { class: "navbar-item", to: "/discord", "Discord" },
            },
            div {
                class: "navbar-end m-2",
                div {
                    class: "buttons",
                    button {
                        class: "button is-primary",
                        onclick: |_| router.navigate_to("/signup"),
                        "Sign up",
                    },
                    button {
                        class: "button is-info",
                        onclick: |_| router.navigate_to("/login"),
                        "Login",
                    },
                },
            },
        },
    })
}

