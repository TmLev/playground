use std::sync::{Arc, Mutex};

use butane::{
    db::{Connection, ConnectionSpec},
    prelude::*,
};

use crate::{
    models::{Artist, NewArtist, UpdatedArtist},
    Error,
};

fn establish_connection() -> Connection {
    butane::db::connect(&ConnectionSpec::load(".butane/connection.json").unwrap()).unwrap()
}

pub struct PgRepo {
    connection: Arc<Mutex<Connection>>,
}

impl PgRepo {
    pub async fn new() -> Self {
        let connection = tokio::task::spawn_blocking(establish_connection)
            .await
            .unwrap();
        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    pub async fn list_artists(&self) -> Result<Vec<Artist>, Error> {
        let connection = self.connection.clone();
        let artists =
            tokio::task::spawn_blocking(move || Artist::query().load(&*connection.lock().unwrap()))
                .await
                .unwrap()?;
        Ok(artists)
    }

    pub async fn retrieve_artist(&self, id: i32) -> Result<Artist, Error> {
        let connection = self.connection.clone();
        let artist =
            tokio::task::spawn_blocking(move || Artist::get(&*connection.lock().unwrap(), id))
                .await
                .unwrap()?;
        Ok(artist)
    }

    pub async fn create_artist(&self, new_artist: NewArtist) -> Result<Artist, Error> {
        let connection = self.connection.clone();
        let artist = tokio::task::spawn_blocking(move || {
            let mut artist = Artist::new(new_artist.name, new_artist.description);
            artist.save(&*connection.lock().unwrap()).map(|_| artist)
        })
        .await
        .unwrap()?;
        Ok(artist)
    }

    pub async fn update_artist(&self, updated_artist: UpdatedArtist) -> Result<Artist, Error> {
        let connection = self.connection.clone();

        let artist = tokio::task::spawn_blocking(move || {
            let artist = Artist::get(&*connection.lock().unwrap(), updated_artist.id);
            if let Err(e) = artist {
                return Err(e);
            }

            let mut artist = artist.unwrap();
            if let Some(name) = updated_artist.name {
                artist.name = name;
            }
            if let Some(description) = updated_artist.description {
                artist.description = description;
            }

            artist.save(&*connection.lock().unwrap()).map(|_| artist)
        })
        .await
        .unwrap()?;

        Ok(artist)
    }

    pub async fn destroy_artist(&self, id: i32) -> Result<(), Error> {
        let connection = self.connection.clone();
        tokio::task::spawn_blocking(move || {
            let artist = Artist::get(&*connection.lock().unwrap(), id);
            if let Err(e) = artist {
                return Err(e);
            }
            artist.unwrap().delete(&*connection.lock().unwrap())
        })
        .await
        .unwrap()?;
        Ok(())
    }
}
