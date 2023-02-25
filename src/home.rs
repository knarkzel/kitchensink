use crate::*;

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            class: "mt-0",
            "Messages",
        },
        ul {
            (1..=10).map(|i| rsx! {
                li {
                    "Item {i}",
                },
            }),
        },                
    })
}