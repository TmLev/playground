use std::sync::Arc;

use tokio::sync::Mutex;

use tonic::{transport::Server, Request, Response, Status};

use rpc::{
    models::{Artist, NewArtist, UpdatedArtist},
    Error, PgRepo,
};

pub mod music {
    tonic::include_proto!("music");
}

pub struct Music {
    repo: Arc<Mutex<PgRepo>>,
}

impl Music {
    async fn new() -> Self {
        let repo = PgRepo::new().await;
        Self {
            repo: Arc::new(Mutex::new(repo)),
        }
    }
}

#[tonic::async_trait]
impl music::music_server::Music for Music {
    async fn list_artists(
        &self,
        request: Request<music::ArtistsListRequest>,
    ) -> Result<Response<music::ArtistsListResponse>, Status> {
        println!("Got a request: {:?}", request);
        Ok(Response::new(
            self.repo.lock().await.list_artists().await.into(),
        ))
    }

    async fn retrieve_artist(
        &self,
        request: Request<music::ArtistRetrieveRequest>,
    ) -> Result<Response<music::ArtistRetrieveResponse>, Status> {
        println!("Got a request: {:?}", request);
        let id = request.into_inner().id;
        Ok(Response::new(
            self.repo.lock().await.retrieve_artist(id).await.into(),
        ))
    }

    async fn create_artist(
        &self,
        request: Request<music::ArtistCreateRequest>,
    ) -> Result<Response<music::ArtistCreateResponse>, Status> {
        println!("Got a request: {:?}", request);
        let new_artist = request.into_inner().into();
        Ok(Response::new(
            self.repo
                .lock()
                .await
                .create_artist(new_artist)
                .await
                .into(),
        ))
    }

    async fn update_artist(
        &self,
        request: Request<music::ArtistUpdateRequest>,
    ) -> Result<Response<music::ArtistUpdateResponse>, Status> {
        println!("Got a request: {:?}", request);
        let updated_artist = request.into_inner().into();
        Ok(Response::new(
            self.repo
                .lock()
                .await
                .update_artist(updated_artist)
                .await
                .into(),
        ))
    }

    async fn destroy_artist(
        &self,
        request: Request<music::ArtistDestroyRequest>,
    ) -> Result<Response<music::ArtistDestroyResponse>, Status> {
        println!("Got a request: {:?}", request);
        let id = request.into_inner().id;
        Ok(Response::new(
            self.repo.lock().await.destroy_artist(id).await.into(),
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let music = Music::new().await;

    println!("Running at {:?}", addr);
    Server::builder()
        .add_service(music::music_server::MusicServer::new(music))
        .serve(addr)
        .await?;

    Ok(())
}

// List

impl From<Result<Vec<Artist>, Error>> for music::ArtistsListResponse {
    fn from(r: Result<Vec<Artist>, Error>) -> Self {
        use music::artists_list_response::Result as R;

        let result = match r {
            Ok(artists) => {
                let artists = music::Artists {
                    artists: artists.into_iter().map(From::from).collect(),
                };
                R::Artists(artists)
            }
            Err(error) => R::Error(error.into()),
        };

        Self {
            result: Some(result),
        }
    }
}

// Retrieve

impl From<Result<Artist, Error>> for music::ArtistRetrieveResponse {
    fn from(r: Result<Artist, Error>) -> Self {
        use music::artist_retrieve_response::Result as R;
        let result = match r {
            Ok(artist) => R::Artist(artist.into()),
            Err(error) => R::Error(error.into()),
        };
        Self {
            result: Some(result),
        }
    }
}

// Create

impl From<Result<Artist, Error>> for music::ArtistCreateResponse {
    fn from(r: Result<Artist, Error>) -> Self {
        use music::artist_create_response::Result as R;
        let result = match r {
            Ok(artist) => R::Artist(artist.into()),
            Err(error) => R::Error(error.into()),
        };
        Self {
            result: Some(result),
        }
    }
}

// Update

impl From<Result<Artist, Error>> for music::ArtistUpdateResponse {
    fn from(r: Result<Artist, Error>) -> Self {
        use music::artist_update_response::Result as R;
        let result = match r {
            Ok(artist) => R::Artist(artist.into()),
            Err(error) => R::Error(error.into()),
        };
        Self {
            result: Some(result),
        }
    }
}

// Destroy

impl From<Result<(), Error>> for music::ArtistDestroyResponse {
    fn from(r: Result<(), Error>) -> Self {
        let error = match r {
            Ok(()) => None,
            Err(error) => Some(error.into()),
        };
        Self { error }
    }
}

// Common

impl From<Artist> for music::Artist {
    fn from(artist: Artist) -> Self {
        Self {
            id: artist.id,
            name: artist.name,
            description: artist.description,
        }
    }
}

impl From<Error> for music::Error {
    fn from(error: Error) -> Self {
        Self {
            details: error.details,
        }
    }
}

impl From<music::ArtistCreateRequest> for NewArtist {
    fn from(request: music::ArtistCreateRequest) -> Self {
        Self {
            name: request.name,
            description: request.description,
        }
    }
}

impl From<music::ArtistUpdateRequest> for UpdatedArtist {
    fn from(request: music::ArtistUpdateRequest) -> Self {
        Self {
            id: request.id,
            name: request.name,
            description: request.description,
        }
    }
}
