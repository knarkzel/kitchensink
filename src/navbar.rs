use crate::*;
use dioxus_router::{use_router, Link};

pub fn NavBar(cx: Scope) -> Element {
    // Hooks
    let router = use_router(cx);

    // User
    let user = use_read(cx, USER);
    let set_user = use_set(cx, USER);
    
    cx.render(rsx! {
        nav {
            class: "navbar has-background-light",
            div {
                class: "navbar-start",
                Link { class: "navbar-item", to: "/", "Home" },
                if user.is_some() {
                    rsx! {
                        Link { class: "navbar-item", to: "/discord", "Discord" },
                    }
                }
            },
            div {
                class: "navbar-end m-2",
                div {
                    class: "buttons",
                    if user.is_none() {
                        rsx! {
                            button {
                                class: "button",
                                onclick: |_| router.navigate_to("/signup"),
                                "Sign up",
                            },
                            button {
                                class: "button",
                                onclick: |_| router.navigate_to("/login"),
                                "Login",
                            },                            
                        }
                    } else {
                        rsx! {
                            button {
                                class: "button",
                                onclick: |_| router.navigate_to("/settings"),
                                "Settings",
                            },
                            button {
                                class: "button",
                                onclick: |_| {
                                    set_user(None);
                                    router.navigate_to("/");
                                },
                                "Logout",
                            },
                        }
                    }
                },
            },                    
        },
    })
}

