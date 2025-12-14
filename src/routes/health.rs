use rocket::serde::{Serialize, json::Json};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Health {
    healthy: bool,
}

#[get("/health")]
pub fn health() -> Json<Health> {
    Json(Health { healthy: true })
}
