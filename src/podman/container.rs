use futures_util::Stream;
use podman_api::Error;
use podman_api::api::Container as PodmanContainer;
use podman_api::conn::TtyChunk;
use podman_api::opts::ContainerLogsOpts;

pub struct Container {
    pub(crate) inner: PodmanContainer,
}

impl Container {
    pub fn id(&self) -> &podman_api::Id {
        self.inner.id()
    }

    pub async fn exists(&self) -> Result<bool, Error> {
        self.inner.exists().await
    }

    pub async fn stop(&self) -> Result<(), Error> {
        self.inner.stop(&Default::default()).await
    }

    pub fn logs(&self) -> impl Stream<Item = Result<TtyChunk, Error>> + '_ {
        let opts = ContainerLogsOpts::builder()
            .follow(true)
            .stdout(true)
            .stderr(true)
            .build();

        self.inner.logs(&opts)
    }
}
