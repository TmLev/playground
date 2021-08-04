use rocket::{get, launch, response::status::NotFound, serde::json::Json};

use diesel::prelude::*;

use rest_api::{models::Artist, schema::artists, ApiError, PgConnection};

#[launch]
fn rocket() -> _ {
    rocket::build()
        // State
        .attach(PgConnection::fairing())
        // Routes
        .mount("/artists", rocket::routes![list, retrieve])
}

#[get("/")]
async fn list(connection: PgConnection) -> Json<Vec<Artist>> {
    connection
        .run(|c| artists::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch artists")
}

#[get("/<id>")]
async fn retrieve(
    connection: PgConnection,
    id: i32,
) -> Result<Json<Artist>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| artists::table.filter(artists::id.eq(id)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}
