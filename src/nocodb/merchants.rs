use crate::nocodb::NocoDB;
use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Merchant {}

impl<'a> NocoDB<'a> {
    pub async fn post_merchant(&self) -> Result<()> {
        Err(anyhow!("TODO - not implemented"))
    }
}
