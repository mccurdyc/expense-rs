use crate::airtable::Airtable;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Merchants {
    pub records: Vec<Merchant>,
}

#[derive(Debug, Deserialize)]
pub struct Merchant {
    pub fields: MerchantDetails,
}

#[derive(Debug, Deserialize)]
pub struct MerchantDetails {
    pub id: Option<String>,
    #[serde(rename(deserialize = "Purchases"))]
    pub purchases: Option<Vec<String>>,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}

impl<'a> Airtable<'a> {
    pub async fn get_merchants(&self) -> Result<Merchants> {
        let res = self
            .client
            .get(self.get_url("Merchants"))
            .bearer_auth(self.api_token)
            .send()
            .await?;
        let merchants: Merchants = res.json().await?;
        Ok(merchants)
    }
}
