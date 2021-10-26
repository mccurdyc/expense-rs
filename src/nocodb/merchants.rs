use crate::nocodb::NocoDB;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Merchant {}

impl<'a> NocoDB<'a> {
    pub async fn post_merchant(&self) -> Result<Merchants> {
        Err("TODO - not implemented")
    }
}
