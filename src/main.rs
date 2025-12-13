use rocket::serde::{Deserialize, Serialize, json::Json};

#[macro_use]
extern crate rocket;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Health {
    healthy: bool,
}

#[get("/health")]
fn health() -> Json<Health> {
    Json(Health { healthy: true })
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
enum Platform {
    GitHub,
    GitLab,
    Bitbucket,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Repo<'r> {
    owner: &'r str,
    name: &'r str,
    platform: Platform,
}

#[post("/register", format = "json", data = "<repo>")]
fn create_validation_runner(repo: Json<Repo<'_>>) -> std::io::Result<()> {
    println!("{:?} {} {}", repo.platform, repo.owner, repo.name);
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![create_validation_runner, health])
}
