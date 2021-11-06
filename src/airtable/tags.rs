use crate::airtable::Airtable;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tags {
    pub records: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub id: Option<String>,
    pub fields: TagDetails,
}

#[derive(Debug, Deserialize)]
pub struct TagDetails {
    #[serde(rename(deserialize = "Purchases"))]
    pub purchases: Option<Vec<String>>,
    #[serde(rename(deserialize = "Name"))]
    pub name: Option<String>,
}

impl<'a> Airtable<'a> {
    // TODO - these could be made generic, let's try.
    pub async fn get_tag(&self, id: &str) -> Result<Tag> {
        let res = self
            .client
            .get(self.get_url(&format!("Tags/{}", id)))
            .bearer_auth(self.api_token)
            .send()
            .await?;
        let tag = res.json().await?;
        Ok(tag)
    }

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
