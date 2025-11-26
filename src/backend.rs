use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        conn
    };
}

#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    }
    Ok(())
}
