use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::i64;

use crate::nocodb::NocoDB;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: Option<String>,
}

impl<'a> NocoDB<'a> {
    pub async fn get_tag(&self, name: &'a str) -> Result<Tag> {
        let res = self
            .client
            .get(self.get_url("tags/findOne"))
            .query(&[("where", format!("(name,eq,{})", name))])
            .header("xc-auth", self.api_token)
            .send()
            .await?;
        let tag = res.json().await?;
        Ok(tag)
    }

    pub async fn add_tag(&self, t: Tag) -> Result<()> {
        self.client
            .post(self.get_url("tags"))
            .header("xc-auth", self.api_token)
            .json(&t)
            .send()
            .await?;
        Ok(())
    }
}
