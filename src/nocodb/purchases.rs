use anyhow::Result;
use serde::Serialize;
use std::{f64, i64};

use crate::nocodb::NocoDB;

#[derive(Debug, Serialize)]
pub struct Purchase {
    pub amount: f64,
    pub date: String,
    pub tag_id: i64,
    pub merchant_id: i64,
}

impl<'a> NocoDB<'a> {
    pub async fn add_purchase(&self, p: Purchase) -> Result<()> {
        self.client
            .post(self.get_url("purchases"))
            .header("xc-auth", self.api_token)
            .json(&p)
            .send()
            .await?;
        Ok(())
    }
}
