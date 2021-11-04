use anyhow::Result;
use serde::Serialize;

use crate::nocodb::NocoDB;

#[derive(Debug, Serialize)]
pub struct Merchant<'a> {
    pub name: &'a str,
}

impl<'a> NocoDB<'a> {
    pub async fn add_merchant(&self, m: Merchant<'a>) -> Result<()> {
        self.client
            .post(self.get_url("merchants"))
            .header("xc-auth", self.api_token)
            .json(&m)
            .send()
            .await?;
        Ok(())
    }
}
