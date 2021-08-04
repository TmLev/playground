use rocket::serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub description: String,
}
