use dioxus::prelude::*;
use serde::Deserialize;
use crate::backend::{save_dog, delete_dog};

static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,
    #[route("/favorites")]
    Favorites,
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::DogView,
                h1 { "🌭 HotDog! " }
            },
            Link { to: Route::Favorites,
                h2 { "♥️ Favorites" }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { id: "not-found",
            h2 { "404 - Page Not Found" }
            p { "The requested path was not found: " }
            // ul {
            //     segments.iter().map(|segment| rsx! {
            //         li { "{segment}" }
            //     })
            // }
        }
    }
}

#[derive(Deserialize, Clone)]
struct DogApi {
    message: String,
}

#[component]
fn Favorites() -> Element {
    let mut favorites = use_resource(super::backend::list_dogs);
    let dogs = favorites.suspend()?;

    rsx! {
            div { id: "favorites",
                div { id: "favorites-container",
                    for (id, url) in dogs().unwrap() {
                        // Render a div for each photo using the dog's ID as the list key
                        div {
                            key: "{id}",
                            class: "favorite-dog",
                            img { src: "{url}" }
                            button {
                                onclick: move |_| async move {
                                    let _ = delete_dog(id).await;
                                    // Refresh the favorites list after deletion
                                    favorites.restart();
                                },
                                "❌"
                            }
                        }
                    }
                }
            }
        }

}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button {
                onclick: move |_| async move {
                    if let Some(current) = img_src.cloned() {
                        img_src.restart();
                        let _ = save_dog(current).await;
                    }
                },
                id: "save",
                "save!"
            }
        }
    }
}
