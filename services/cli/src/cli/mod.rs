use std::convert::TryFrom;

use ::clap::Parser;

use crate::cli::clap::CliBencher;
use crate::BencherError;

pub mod adapter;
pub mod benchmark;
pub mod clap;
pub mod sub;
pub mod testbed;
pub mod wide;

use sub::map_sub;
use sub::Sub;
use sub::SubCmd;
use wide::Wide;

#[derive(Debug)]
pub struct Bencher {
    wide: Wide,
    sub: Option<Sub>,
}

impl TryFrom<CliBencher> for Bencher {
    type Error = BencherError;

    fn try_from(bencher: CliBencher) -> Result<Self, Self::Error> {
        Ok(Self {
            wide: Wide::try_from(bencher.wide)?,
            sub: map_sub(bencher.sub)?,
        })
    }
}

impl Bencher {
    pub fn new() -> Result<Self, BencherError> {
        let args = CliBencher::parse();
        Self::try_from(args)
    }

    pub async fn exec(&self) -> Result<(), BencherError> {
        if let Some(sub) = &self.sub {
            sub.exec(&self.wide).await
        } else {
            self.ping().await
        }
    }

    // TODO actually implement this ping / check auth endpoint
    pub async fn ping(&self) -> Result<(), BencherError> {
        let client = reqwest::Client::new();
        let url = self.wide.url.join("/v0/ping")?.to_string();
        let res = client.get(&url).send().await?;
        println!("{res:?}");
        Ok(())
    }
}
