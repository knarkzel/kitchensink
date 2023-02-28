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
                class: "navbar-end",
                if user.is_some() {
                    rsx! {
                        Link { class: "navbar-item", to: "/settings", "Settings" },
                        a {
                            class: "navbar-item",
                            onclick: |_| {
                                set_user(None);
                                LocalStorage::delete("user");
                                router.navigate_to("/");
                            },
                            "Logout"
                        },
                    }
                } else {
                    rsx! {
                        Link { class: "navbar-item", to: "/account", "Account" },
                    }
                }
            },
        },
    })
}
