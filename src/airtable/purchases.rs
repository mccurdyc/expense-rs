use crate::airtable::Airtable;
use anyhow::Result;
use serde::Deserialize;
use std::f64;

#[derive(Debug, Deserialize)]
pub struct Purchases {
    pub records: Vec<Purchase>,
    offset: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Purchase {
    pub fields: PurchaseDetails,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseDetails {
    #[serde(rename(deserialize = "Amount"))]
    pub amount: Option<f64>,
    #[serde(rename(deserialize = "Tag"))]
    pub tags: Option<Vec<String>>,
    #[serde(rename(deserialize = "Merchant"))]
    pub merchant: Option<Vec<String>>,
    #[serde(rename(deserialize = "Date"))]
    pub datestr: Option<String>,
}

impl<'a> Airtable<'a> {
    pub async fn get_purchases(&self) -> Result<Vec<Purchase>> {
        let mut result: Vec<Purchase> = vec![];
        let mut offset: Option<String> = None;

        let mut purchases = self.get_purchases_page(offset).await?;
        result.append(&mut purchases.records);
        offset = purchases.offset;

        // TODO - there's got to be a better pattern since I'm not even using the `Some(o)` value.
        while let Some(o) = offset {
            purchases = self.get_purchases_page(Some(o)).await?;
            result.append(&mut purchases.records);
            offset = purchases.offset;
        }

        Ok(result)
    }

    async fn get_purchases_page(&self, offset: Option<String>) -> Result<Purchases> {
        let mut req = self
            .client
            .get(self.get_url("Purchases"))
            .bearer_auth(self.api_token);

        if let Some(offset) = offset {
            req = req.query(&[("offset", offset)]);
        }

        // TODO - debug
        // let res2 = req.try_clone().unwrap().send().await?.text().await?;
        // println!("{}", res2);

        let res = req.send().await?;
        let purchases: Purchases = res.json().await?;
        Ok(purchases)
    }
}
