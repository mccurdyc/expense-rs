use anyhow::Result;
use serde::Serialize;

use crate::nocodb::NocoDB;

#[derive(Debug, Serialize)]
pub struct Tag<'a> {
    pub name: &'a str,
}

impl<'a> NocoDB<'a> {
    pub async fn add_tag(&self, t: Tag<'a>) -> Result<()> {
        self.client
            .post(self.get_url("tags"))
            .header("xc-auth", self.api_token)
            .json(&t)
            .send()
            .await?;
        Ok(())
    }
}
