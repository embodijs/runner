use rocket::serde::{Serialize, json::Json};
use rocket::{Route, routes};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Health {
    healthy: bool,
}

#[get("/health")]
fn health() -> Json<Health> {
    Json(Health { healthy: true })
}

pub fn routes() -> Vec<Route> {
    routes![health]
}
