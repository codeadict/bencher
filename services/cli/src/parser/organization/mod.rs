use bencher_json::{NonEmpty, ResourceId, Slug};
use clap::{Parser, Subcommand, ValueEnum};

use crate::parser::CliBackend;

pub mod member;
#[cfg(feature = "plus")]
pub mod plan;
#[cfg(feature = "plus")]
pub mod usage;

#[cfg(feature = "plus")]
use self::{plan::CliOrganizationPlan, usage::CliOrganizationUsage};

use super::CliPagination;

#[derive(Subcommand, Debug)]
pub enum CliOrganization {
    /// List organizations
    #[clap(alias = "ls")]
    List(CliOrganizationList),
    /// Create an organization
    #[clap(alias = "add")]
    Create(CliOrganizationCreate),
    /// View an organization
    #[clap(alias = "cat")]
    View(CliOrganizationView),
    /// Update an organization
    #[clap(alias = "edit")]
    Update(CliOrganizationUpdate),
    /// Check organization permission
    Allowed(CliOrganizationAllowed),

    #[cfg(feature = "plus")]
    /// Organization metered subscription plan
    #[clap(subcommand)]
    Plan(CliOrganizationPlan),

    #[cfg(feature = "plus")]
    /// Check organization metrics usage
    Usage(CliOrganizationUsage),
}

#[derive(Parser, Debug)]
pub struct CliOrganizationList {
    /// Organization name
    #[clap(long)]
    pub name: Option<NonEmpty>,

    #[clap(flatten)]
    pub pagination: CliPagination<CliOrganizationsSort>,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "snake_case")]
pub enum CliOrganizationsSort {
    /// Name of the organization
    Name,
}

#[derive(Parser, Debug)]
pub struct CliOrganizationCreate {
    /// Organization name
    pub name: NonEmpty,

    /// Organization slug
    #[clap(long)]
    pub slug: Option<Slug>,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Parser, Debug)]
pub struct CliOrganizationView {
    /// Organization slug or UUID
    pub organization: ResourceId,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Parser, Debug)]
pub struct CliOrganizationUpdate {
    /// Organization slug or UUID
    pub organization: ResourceId,

    /// New organization name
    #[clap(long)]
    pub name: Option<NonEmpty>,

    /// New organization slug
    #[clap(long)]
    pub slug: Option<Slug>,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Parser, Debug)]
pub struct CliOrganizationAllowed {
    /// Organization slug or UUID
    pub organization: ResourceId,

    /// Organization permission
    #[clap(long)]
    pub perm: CliOrganizationPermission,

    #[clap(flatten)]
    pub backend: CliBackend,
}

/// Organization permission
#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "snake_case")]
pub enum CliOrganizationPermission {
    View,
    Create,
    Edit,
    Delete,
    Manage,
    ViewRole,
    CreateRole,
    EditRole,
    DeleteRole,
}
