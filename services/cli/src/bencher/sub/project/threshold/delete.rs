use std::convert::TryFrom;

use async_trait::async_trait;
use bencher_json::{JsonEmpty, ResourceId};
use uuid::Uuid;

use crate::{
    bencher::{backend::Backend, sub::SubCmd},
    parser::project::threshold::CliThresholdDelete,
    CliError,
};

#[derive(Debug)]
pub struct Delete {
    pub project: ResourceId,
    pub threshold: Uuid,
    pub backend: Backend,
}

impl TryFrom<CliThresholdDelete> for Delete {
    type Error = CliError;

    fn try_from(delete: CliThresholdDelete) -> Result<Self, Self::Error> {
        let CliThresholdDelete {
            project,
            threshold,
            backend,
        } = delete;
        Ok(Self {
            project,
            threshold,
            backend: backend.try_into()?,
        })
    }
}

#[async_trait]
impl SubCmd for Delete {
    async fn exec(&self) -> Result<(), CliError> {
        let _: JsonEmpty = self
            .backend
            .send_with(
                |client| async move {
                    client
                        .proj_threshold_delete()
                        .project(self.project.clone())
                        .threshold(self.threshold)
                        .send()
                        .await
                },
                true,
            )
            .await?;
        Ok(())
    }
}
