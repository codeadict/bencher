use std::convert::TryFrom;

use async_trait::async_trait;
use bencher_json::{JsonNewToken, ResourceId};

use crate::{
    bencher::{backend::Backend, sub::SubCmd, wide::Wide},
    cli::token::CliTokenCreate,
    BencherError,
};

const BRANCHES_PATH: &str = "/v0/tokens";

#[derive(Debug)]
pub struct Create {
    pub user: ResourceId,
    pub ttl: u64,
    pub name: String,
    pub backend: Backend,
}

impl TryFrom<CliTokenCreate> for Create {
    type Error = BencherError;

    fn try_from(create: CliTokenCreate) -> Result<Self, Self::Error> {
        let CliTokenCreate {
            user,
            ttl,
            name,
            backend,
        } = create;
        Ok(Self {
            user,
            ttl,
            name,
            backend: backend.try_into()?,
        })
    }
}

#[async_trait]
impl SubCmd for Create {
    async fn exec(&self, _wide: &Wide) -> Result<(), BencherError> {
        let token = JsonNewToken {
            user: self.user.clone(),
            ttl: self.ttl,
            name: self.name.clone(),
        };
        self.backend.post(BRANCHES_PATH, &token).await?;
        Ok(())
    }
}