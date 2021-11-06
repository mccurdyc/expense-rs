use crate::airtable::Airtable;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Merchants {
    pub records: Vec<Merchant>,
    offset: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Merchant {
    pub id: Option<String>,
    pub fields: MerchantDetails,
}

#[derive(Debug, Deserialize)]
pub struct MerchantDetails {
    pub id: Option<String>,
    #[serde(rename(deserialize = "Purchases"))]
    pub purchases: Option<Vec<String>>,
    #[serde(rename(deserialize = "Name"))]
    pub name: Option<String>,
}

impl<'a> Airtable<'a> {
    pub async fn get_merchant(&self, id: &str) -> Result<Merchant> {
        let res = self
            .client
            .get(self.get_url(&format!("Merchants/{}", id)))
            .bearer_auth(self.api_token)
            .send()
            .await?;
        let merchant = res.json().await?;
        Ok(merchant)
    }

    pub async fn get_merchants(&self) -> Result<Vec<Merchant>> {
        let mut result: Vec<Merchant> = vec![];
        let mut offset: Option<String> = None;

        let mut merchants = self.get_merchants_page(offset).await?;
        result.append(&mut merchants.records);
        offset = merchants.offset;

        // TODO - there's got to be a better pattern since I'm not even using the `Some(o)` value.
        while let Some(o) = offset {
            merchants = self.get_merchants_page(Some(o)).await?;
            result.append(&mut merchants.records);
            offset = merchants.offset;
        }

        Ok(result)
    }

    async fn get_merchants_page(&self, offset: Option<String>) -> Result<Merchants> {
        let mut req = self
            .client
            .get(self.get_url("Merchants"))
            .bearer_auth(self.api_token);

        if let Some(offset) = offset {
            req = req.query(&[("offset", offset)]);
        }

        let res = req.send().await?;
        let merchants = res.json().await?;
        Ok(merchants)
    }
}
