use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{f64, i64};

use crate::nocodb::NocoDB;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Purchase {
    pub id: Option<i64>,
    pub amount: Option<f64>,
    pub date: Option<String>,
    // TODO: this isn't a Vec<i64> it's an object
    // purchase resp: "{\"id\":1,\"date\":\"2021-08-08T08:29:00.000Z\",\"amount\":152.45,\"tagsMMList\":[{\"nc_k11i__purchases_nc_k11i__purchases_p_id\":1,\"id\":1,\"name\":\"house\"}],\"merchantsMMList\":[{\"nc_k11i__purchases_nc_k11i__purchases_p_id\":1,\"id\":1,\"name\":\"dot\"}]}"
    //
    // #[serde(rename(deserialize = "tagsMMList"))]
    // pub tags: Option<Vec<i64>>,
    // #[serde(rename(deserialize = "merchantsMMList"))]
    // pub merchants: Option<Vec<i64>>,
}

#[derive(Debug, Serialize)]
struct PurchaseAssociation {
    #[serde(rename(serialize = "purchasesPId"))]
    purchase_id: i64,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename(serialize = "merchantsCId")
    )]
    merchant_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "tagsCId"))]
    tag_id: Option<i64>,
}

impl<'a> NocoDB<'a> {
    pub async fn add_purchase(&self, p: Purchase) -> Result<Purchase> {
        let resp = self
            .client
            .post(self.get_url("purchases"))
            .header("xc-auth", self.api_token)
            .json(&p)
            .send()
            .await?
            .json::<Purchase>()
            .await?;
        Ok(resp)
    }

    pub async fn associate_tag(&self, purchase_id: i64, tag_id: i64) -> Result<()> {
        let body = PurchaseAssociation {
            purchase_id,
            tag_id: Some(tag_id),
            merchant_id: None,
        };
        println!("associate_tag body: {:?}", serde_json::to_string(&body)?);
        self.client
            .post(self.get_url("m2mpurchases_tags"))
            .header("xc-auth", self.api_token)
            .json(&body)
            .send()
            .await?;
        // TODO: handle status 400
        // println!("associate_tag resp: {:?}", resp);
        Ok(())
    }

    pub async fn associate_merchant(&self, purchase_id: i64, merchant_id: i64) -> Result<()> {
        let body = PurchaseAssociation {
            purchase_id,
            merchant_id: Some(merchant_id),
            tag_id: None,
        };
        println!(
            "associate_merchant body: {:?}",
            serde_json::to_string(&body)?
        );
        self.client
            .post(self.get_url("m2mpurchases_merchants"))
            .header("xc-auth", self.api_token)
            .json(&body)
            .send()
            .await?;
        // TODO: handle status 400
        // println!("associate_merchant resp: {:?}", resp);
        Ok(())
    }
}
