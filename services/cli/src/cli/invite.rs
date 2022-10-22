use bencher_json::ResourceId;
use clap::{Parser, ValueEnum};

use super::CliBackend;

#[derive(Parser, Debug)]
pub struct CliInvite {
    /// Name of user for invitation (optional)
    #[clap(long)]
    pub name: Option<String>,

    /// Email for the invitation
    #[clap(long)]
    pub email: String,

    /// Organization slug or UUID
    #[clap(long)]
    pub org: ResourceId,

    /// Organization role
    #[clap(value_enum, long)]
    pub role: CliInviteRole,

    #[clap(flatten)]
    pub backend: CliBackend,
}

/// Role within the organization
#[derive(ValueEnum, Debug, Clone)]
pub enum CliInviteRole {
    Member,
    Leader,
}
