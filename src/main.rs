#[macro_use]
extern crate rocket;

mod podman;
mod routes;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let podman = podman::Podman::new(std::env::var("PODMAN_HOST").expect("PODMAN_HOST required"))
        .expect("Failed to connect to Podman");

    rocket::build()
        .manage(podman)
        .mount("/", routes::health::routes())
        .mount("/embodi/config", routes::embodi_config::routes())
}
