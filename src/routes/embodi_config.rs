use crate::podman::{Podman, RunOpts};
use futures_util::StreamExt;
use podman_api::conn::TtyChunk;
use rocket::http::Status;
use rocket::response::status::{Custom, NotFound};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::Serialize;
use rocket::serde::{Deserialize, json::Json};
use rocket::{Route, State, routes};
use uuid::Uuid;

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
    token: &'r str,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RegisterRequestData<'r> {
    version: &'r str,
    repo: Repo<'r>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct RegisterResponse {
    id: String,
    key: String,
}

#[post("/register", format = "json", data = "<data>")]
async fn create_validation_runner(
    podman: &State<Podman>,
    data: Json<RegisterRequestData<'_>>,
) -> Result<Json<RegisterResponse>, Custom<String>> {
    let image = format!("alpine:{}", data.version);
    let key = Uuid::new_v4().to_string();
    let platform = format!("{:?}", data.repo.platform);
    let prefix = format!("[{}]", key);
    let env = [
        ("PLATFORM", platform.as_str()),
        ("OWNER", data.repo.owner),
        ("NAME", data.repo.name),
        ("TOKEN", data.repo.token),
        ("PREFIX", prefix.as_str()),
    ];
    let opts = RunOpts::new(&image, Some(&env), Some(false));

    let container = podman
        .run(&opts)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    let id = container.id().to_string();

    Ok(Json(RegisterResponse { id, key }))
}

#[get("/<id>/<key>")]
async fn get_stream(
    podman: &State<Podman>,
    id: &str,
    key: &str,
) -> Result<EventStream![], NotFound<String>> {
    let container = podman.container(id);
    let key = key.to_string();

    if !container.exists().await.unwrap_or(false) {
        return Err(NotFound(format!("Container {} not found", id)));
    }

    Ok(EventStream! {
        let mut logs = container.logs();

        while let Some(chunk) = logs.next().await {
            match chunk {
                Ok(TtyChunk::StdOut(bytes)) => {
                    let line = String::from_utf8_lossy(&bytes);
                    let prefix = format!("[{}]", key);
                    if let Some(msg) = line.strip_prefix(&prefix) {
                        yield Event::data(msg.trim().to_string());
                    }
                }
                Ok(TtyChunk::StdErr(bytes)) => {
                    yield Event::data(String::from_utf8_lossy(&bytes).to_string())
                        .event("stderr");
                }
                Err(_) => break,
                _ => {}
            }
        }

        println!("Stream closed");
    })
}

pub fn routes() -> Vec<Route> {
    routes![create_validation_runner, get_stream]
}
