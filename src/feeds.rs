use crate::*;

pub fn Index(cx: Scope) -> Element {
    // User
    let settings = use_read(cx, SETTINGS);
    let feeds = use_state(cx, || {});

    cx.render(rsx! {
        h1 {
            class: "mt-0",
            "Feeds",
        },
        p {
            "Your feed is currently empty.",
        },
    })
}
