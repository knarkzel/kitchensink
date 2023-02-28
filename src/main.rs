use kitchensink::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    // Logger
    wasm_logger::init(wasm_logger::Config::default());

    // Setup fermi
    use_init_atom_root(cx);

    // Setup local storage
    if let Ok(user) = LocalStorage::get::<SupabaseUser>("user") {
        let set_user = use_set(cx, USER);
        set_user(Some(user));
    }
    
    if let Ok(settings) = LocalStorage::get::<Settings>("settings") {
        let set_settings = use_set(cx, SETTINGS);
        set_settings(settings);
    }

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
                    Route { to: "/settings", settings::Index {} },
                    Route { to: "/feeds", feeds::Index {} },
                },
            },
        },
    })
}
