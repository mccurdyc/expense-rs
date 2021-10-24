use crate::airtable::Airtable;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tags {
    pub records: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub id: String,
    pub fields: TagDetails,
}

#[derive(Debug, Deserialize)]
pub struct TagDetails {
    #[serde(rename(deserialize = "Purchases"))]
    pub purchases: Vec<String>,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}

impl<'a> Airtable<'a> {
    pub async fn get_tags(&self) -> Result<Tags> {
        let res = self
            .client
            .get(self.get_url("Tags"))
            .bearer_auth(self.api_token)
            .send()
            .await?;
        let tags: Tags = res.json().await?;
        Ok(tags)
    }
}
