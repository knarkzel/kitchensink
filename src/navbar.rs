use crate::*;

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
                        Link { class: "navbar-item", to: "/feeds", "Feeds" },
                    }
                }
            },
            div {
                class: "navbar-end m-2",
                div {
                    class: "buttons",
                    if user.is_some() {
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
                                    LocalStorage::delete("user");
                                    router.navigate_to("/");
                                },
                                "Logout",
                            },
                        }
                    } else {
                        rsx! {
                            button {
                                class: "button",
                                onclick: |_| router.navigate_to("/account"),
                                "Account",
                            },
                        }
                    }
                },
            },
        },
    })
}
