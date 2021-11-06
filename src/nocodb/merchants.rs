use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::nocodb::NocoDB;

#[derive(Debug, Serialize, Deserialize)]
pub struct Merchant {
    pub id: Option<i64>,
    pub name: String,
}

impl<'a> NocoDB<'a> {
    pub async fn get_merchant(&self, name: &'a str) -> Result<Merchant> {
        let res = self
            .client
            .get(self.get_url("merchants/findOne"))
            .query(&[("where", format!("(name,eq,{}", name))])
            .header("xc-auth", self.api_token)
            .send()
            .await?;
        let merchant = res.json().await?;
        Ok(merchant)
    }

    pub async fn add_merchant(&self, m: Merchant) -> Result<()> {
        self.client
            .post(self.get_url("merchants"))
            .header("xc-auth", self.api_token)
            .json(&m)
            .send()
            .await?;
        Ok(())
    }
}
