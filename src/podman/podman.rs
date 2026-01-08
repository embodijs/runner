use podman_api::Error;
use podman_api::Podman as PodmanClient;
use podman_api::opts::ContainerCreateOpts;

use crate::podman::container::Container;

pub struct Podman {
    inner: PodmanClient,
}

impl Podman {
    pub fn new(uri: String) -> Result<Self, Error> {
        Ok(Self {
            inner: PodmanClient::new(uri)?,
        })
    }

    pub async fn run(&self, opts: &ContainerCreateOpts) -> Result<Container, Error> {
        let created = self.inner.containers().create(&opts).await?;
        let id = created.id.clone();

        self.inner.containers().get(&id).start(None).await?;

        Ok(Container {
            inner: self.inner.containers().get(&id),
        })
    }

    pub fn container(&self, id: &str) -> Container {
        Container {
            inner: self.inner.containers().get(id),
        }
    }
}
