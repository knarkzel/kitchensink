use crate::*;

pub fn Index(cx: Scope) -> Element {
    // User
    let user = use_read(cx, USER);

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
