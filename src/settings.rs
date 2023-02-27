use crate::*;

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            class: "m-0",
            "Settings",
        },
        div {
            class: "box p-4 mt-2",
            h2 {
                id: "feeds mt-0",
                "Feeds",
            },
            textarea {
                class: "textarea",
                placeholder: "RSS or Atom feeds",
            },
            button {
                class: "button mt-2",
                "Save",
            },            
        }
    })
}
