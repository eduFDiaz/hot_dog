use dioxus::{logger::tracing, prelude::*};
use crate::dioxus_fullstack::Json;
use serde::*;

static CSS: Asset = asset!("/assets/main.css");

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        // Create a new axum router for our Dioxus app
        let router = dioxus::server::router(App);

        // .. customize it however you want ..

        // And then return it
        Ok(router)
    })
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SaveDogArgs {
    image: String,
}

#[server]
async fn save_dog_in_db(image: String) -> Result<()> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

#[post("/api/save_dog")]
async fn save_dog(Json(args): Json<SaveDogArgs>) -> Result<()> {
    tracing::info!("Received image to save: {}", args.image);

    // use std::io::Write;

    // let mut file = std::fs::OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .create(true)
    //     .open("dogs.txt")
    //     .unwrap();

    // // And then write a newline to it with the image url
    // file.write_fmt(format_args!("{}\n", args.image));

    save_dog_in_db(args.image).await?;

    Ok(())
}

#[derive(Deserialize)]
struct DogApi {
    message: String,
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
    async fn save_dog(image: String) -> Result<()> {
    reqwest::Client::new()
        .post("http://localhost:8080/api/save_dog")
        .json(&SaveDogArgs { image })
        .send()
        .await?;
    Ok(())
}

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button {
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = save_dog(current).await;
                },
                id: "save",
                "save!"
            }
        }

    }
}
