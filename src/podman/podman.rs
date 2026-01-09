use futures_util::{StreamExt, TryStreamExt};
use podman_api::Error;
use podman_api::Podman as PodmanClient;
use podman_api::opts::{ContainerCreateOpts, PullOpts};

use crate::podman::container::Container;

pub struct RunOpts<'a> {
    pub image: &'a str,
    pub env: Option<&'a [(&'a str, &'a str)]>,
    pub remove: bool,
}

impl<'a> RunOpts<'a> {
    pub fn new(
        image: &'a str,
        env: Option<&'a [(&'a str, &'a str)]>,
        remove: Option<bool>,
    ) -> Self {
        Self {
            image,
            env,
            remove: remove.unwrap_or(false),
        }
    }

    fn to_create_opts(&self) -> ContainerCreateOpts {
        ContainerCreateOpts::builder()
            .image(self.image)
            .remove(self.remove)
            .env(self.env.map(|e| e.iter().copied()).unwrap_or_default())
            .build()
    }
}

pub struct Podman {
    inner: PodmanClient,
}

impl Podman {
    pub fn new(uri: String) -> Result<Self, Error> {
        Ok(Self {
            inner: PodmanClient::new(uri)?,
        })
    }

    pub async fn run(&self, opts: &RunOpts<'_>) -> Result<Container, Error> {
        let create_opts = opts.to_create_opts();
        let created = match self.inner.containers().create(&create_opts).await {
            Ok(c) => c,
            Err(Error::Fault { code, .. }) if code.as_u16() == 404 => {
                self.pull(&opts.image).await?;
                self.inner.containers().create(&create_opts).await?
            }
            Err(e) => return Err(e),
        };

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

    pub async fn pull(&self, image: &str) -> Result<(), Error> {
        let opts = PullOpts::builder().reference(image).build();
        self.inner
            .images()
            .pull(&opts)
            .try_for_each(|_| async { Ok(()) })
            .await
    }
}
