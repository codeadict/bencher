use std::convert::TryFrom;

use async_trait::async_trait;
use bencher_client::types::JsonUpdateTestbed;
use bencher_json::{JsonTestbed, NonEmpty, ResourceId, Slug};

use crate::{
    bencher::{backend::Backend, sub::SubCmd},
    parser::project::testbed::CliTestbedUpdate,
    CliError,
};

#[derive(Debug, Clone)]
pub struct Update {
    pub project: ResourceId,
    pub testbed: ResourceId,
    pub name: Option<NonEmpty>,
    pub slug: Option<Slug>,
    pub backend: Backend,
}

impl TryFrom<CliTestbedUpdate> for Update {
    type Error = CliError;

    fn try_from(create: CliTestbedUpdate) -> Result<Self, Self::Error> {
        let CliTestbedUpdate {
            project,
            testbed,
            name,
            slug,
            backend,
        } = create;
        Ok(Self {
            project,
            testbed,
            name,
            slug,
            backend: backend.try_into()?,
        })
    }
}

impl From<Update> for JsonUpdateTestbed {
    fn from(create: Update) -> Self {
        let Update { name, slug, .. } = create;
        Self {
            name: name.map(Into::into),
            slug: slug.map(Into::into),
        }
    }
}

#[async_trait]
impl SubCmd for Update {
    async fn exec(&self) -> Result<(), CliError> {
        let _: JsonTestbed = self
            .backend
            .send_with(
                |client| async move {
                    client
                        .proj_testbed_patch()
                        .project(self.project.clone())
                        .testbed(self.testbed.clone())
                        .body(self.clone())
                        .send()
                        .await
                },
                true,
            )
            .await?;
        Ok(())
    }
}
