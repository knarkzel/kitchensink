use communicate::*;
use dioxus_router::{Router, Route};

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    // Setup fermi
    use_init_atom_root(cx);
    
    cx.render(rsx! {
        main {
            Router {
                navbar::NavBar {},
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
