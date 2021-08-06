pub mod music {
    tonic::include_proto!("music");
}

use music::artist_create_response::Result as R;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://[::1]:50051";
    let mut client = music::music_client::MusicClient::connect(addr).await?;

    print!("Sending `create` request...");
    let request = tonic::Request::new(music::ArtistCreateRequest {
        name: "Rick Roll".into(),
        description: "Never Gonna Give You Up".into(),
    });
    let response = client.create_artist(request).await?.into_inner();
    println!(" Got {:?}", response);

    let rick_id = match response.result.unwrap() {
        R::Artist(a) => a.id,
        R::Error(e) => panic!("{}", e.details),
    };

    print!("Sending one more `create` request...");
    let request = tonic::Request::new(music::ArtistCreateRequest {
        name: "Gorillaz".into(),
        description: "19/2000".into(),
    });
    let response = client.create_artist(request).await?.into_inner();
    println!(" Got {:?}", response);

    let gorillaz_id = match response.result.unwrap() {
        R::Artist(a) => a.id,
        R::Error(e) => panic!("{}", e.details),
    };

    print!("Sending `list` request...");
    let request = tonic::Request::new(music::ArtistsListRequest {});
    let response = client.list_artists(request).await?.into_inner();
    println!(" Got {:?}", response);

    print!("Sending `retrieve` request...");
    let request = tonic::Request::new(music::ArtistRetrieveRequest { id: rick_id });
    let response = client.retrieve_artist(request).await?.into_inner();
    println!(" Got {:?}", response);

    print!("Sending `update` request...");
    let request = tonic::Request::new(music::ArtistUpdateRequest {
        id: rick_id,
        name: Some("Rick Astley".into()),
        description: None,
    });
    let response = client.update_artist(request).await?.into_inner();
    println!(" Got {:?}", response);

    print!("Sending `retrieve` request...");
    let request = tonic::Request::new(music::ArtistRetrieveRequest { id: rick_id });
    let response = client.retrieve_artist(request).await?.into_inner();
    println!(" Got {:?}", response);

    print!("Sending `destroy` request...");
    let request = tonic::Request::new(music::ArtistDestroyRequest { id: gorillaz_id });
    let response = client.destroy_artist(request).await?.into_inner();
    println!(" Got {:?}", response);

    print!("Sending final `list` request...");
    let request = tonic::Request::new(music::ArtistsListRequest {});
    let response = client.list_artists(request).await?.into_inner();
    println!(" Got {:?}", response);

    Ok(())
}
