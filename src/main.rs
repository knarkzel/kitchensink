use communicate::*;
use dioxus_router::{Router, Route, Link, use_router};

fn main() {
    dioxus_web::launch(app);
}

fn NavBar(cx: Scope) -> Element {
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
                class: "navbar-end mr-2",
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

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            Router {
                NavBar {},
                div {
                    class: "content p-4",
                    Route { to: "/", home::Index {} },
                    Route { to: "/login", login::Index {} },
                    Route { to: "/signup", signup::Index {} },
                    Route { to: "/discord", discord::Index {} },
                },
            },
        },
    })
}
