use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{f64, i64};

use crate::nocodb::NocoDB;

#[derive(Debug, Deserialize, Serialize)]
pub struct Purchase {
    pub id: Option<i64>,
    pub amount: Option<f64>,
    pub date: Option<String>,
    #[serde(rename(deserialize = "tagsMMList"))]
    pub tags: Option<Vec<i64>>,
    #[serde(rename(deserialize = "merchantsMMList"))]
    pub merchants: Option<Vec<i64>>,
}

#[derive(Debug, Serialize)]
struct PurchaseAssociation {
    purchase_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    merchant_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_id: Option<i64>,
}

impl<'a> NocoDB<'a> {
    pub async fn add_purchase(&self, p: Purchase) -> Result<Purchase> {
        let purchase = self
            .client
            .post(self.get_url("purchases"))
            .header("xc-auth", self.api_token)
            .json(&p)
            .send()
            .await?
            .json::<Purchase>()
            .await?;
        Ok(purchase)
    }

    pub async fn associate_tag(&self, purchase_id: i64, tag_id: i64) -> Result<()> {
        let body = PurchaseAssociation {
            purchase_id,
            tag_id: Some(tag_id),
            merchant_id: None,
        };
        self.client
            .post(self.get_url("m2mpurchases_tags"))
            .header("xc-auth", self.api_token)
            .json(&body)
            .send()
            .await?;
        Ok(())
    }

    pub async fn associate_merchant(&self, purchase_id: i64, merchant_id: i64) -> Result<()> {
        let body = PurchaseAssociation {
            purchase_id,
            merchant_id: Some(merchant_id),
            tag_id: None,
        };
        self.client
            .post(self.get_url("m2mpurchases_merchants"))
            .header("xc-auth", self.api_token)
            .json(&body)
            .send()
            .await?;
        Ok(())
    }
}
