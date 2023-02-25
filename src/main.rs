use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "container",
            h1 {
                class: "title is-1 my-4 has-text-centered",
                "Communicate",
            },
            div {
                class: "content",
                ul {
                    (1..=10).map(|i| rsx! {
                        li {
                            "Item {i}",
                        },
                    }),
                },                
            },
        }
    })
}
