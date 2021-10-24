use crate::airtable::Airtable;
use anyhow::Result;
use serde::Deserialize;
use std::f64;

#[derive(Debug, Deserialize)]
pub struct Purchases {
    pub records: Vec<Purchase>,
}

#[derive(Debug, Deserialize)]
pub struct Purchase {
    pub fields: PurchaseDetails,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseDetails {
    #[serde(rename(deserialize = "Amount"))]
    pub amount: f64,
    #[serde(rename(deserialize = "Tag"))]
    pub tag: Vec<String>,
    #[serde(rename(deserialize = "Merchant"))]
    pub merchant: Vec<String>,
    #[serde(rename(deserialize = "Date"))]
    pub datestr: String,
}

impl<'a> Airtable<'a> {
    pub async fn get_purchases(&self) -> Result<Purchases> {
        let res = self
            .client
            .get(self.get_url("Purchases"))
            .bearer_auth(self.api_token)
            .send()
            .await?;
        let purchases: Purchases = res.json().await?;
        Ok(purchases)
    }
}
