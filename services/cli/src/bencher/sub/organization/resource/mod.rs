use async_trait::async_trait;

use crate::{bencher::sub::SubCmd, cli::organization::CliOrganization, CliError};

mod allowed;
mod create;
mod list;
mod view;

#[derive(Debug)]
pub enum Organization {
    List(list::List),
    Create(create::Create),
    View(view::View),
    Allowed(allowed::Allowed),
}

impl TryFrom<CliOrganization> for Organization {
    type Error = CliError;

    fn try_from(branch: CliOrganization) -> Result<Self, Self::Error> {
        Ok(match branch {
            CliOrganization::List(list) => Self::List(list.try_into()?),
            CliOrganization::Create(create) => Self::Create(create.try_into()?),
            CliOrganization::View(view) => Self::View(view.try_into()?),
            CliOrganization::Allowed(allowed) => Self::Allowed(allowed.try_into()?),
        })
    }
}

#[async_trait]
impl SubCmd for Organization {
    async fn exec(&self) -> Result<(), CliError> {
        match self {
            Self::List(list) => list.exec().await,
            Self::Create(create) => create.exec().await,
            Self::View(view) => view.exec().await,
            Self::Allowed(allowed) => allowed.exec().await,
        }
    }
}
