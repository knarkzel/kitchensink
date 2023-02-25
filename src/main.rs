use communicate::*;
use dioxus_router::{Router, Route, Link};

fn main() {
    dioxus_web::launch(app);
}

fn NavBar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "navbar has-background-light",
            div {
                class: "navbar-start",
                Link { class: "navbar-item", to: "/", "Home" },
                Link { class: "navbar-item", to: "/discord", "Discord" },
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
                    Route { to: "/discord", discord::Index {} },
                },
            },
        },
    })
}
