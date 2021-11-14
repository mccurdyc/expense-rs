use anyhow::Error;
use std::env;

mod airtable;
mod nocodb;

use crate::airtable::Airtable;
use crate::nocodb::NocoDB;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let at_api_token = env::var("AIRTABLE_API_TOKEN").expect("AIRTABLE_API_TOKEN env not set");
    let at_base_id = env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID env not set");

    let nc_host = env::var("NOCODB_HOST").expect("NOCODB_HOST env not set");
    let nc_project_id = env::var("NOCODB_PROJECT_ID").expect("NOCODB_PROJECT_ID env not set");
    let nc_api_token = env::var("NOCODB_API_TOKEN").expect("NOCODB_API_TOKEN env not set");

    let table = Airtable::new(&at_api_token, &at_base_id)?;
    let db = NocoDB::new(&nc_host, &nc_project_id, &nc_api_token)?;

    let tags = table.get_tags().await.expect("failed to retrieve tags");
    let mut recs = tags
        .records
        .into_iter()
        .filter(|t| t.id.is_some() && t.fields.name.is_some() && t.fields.purchases.is_some())
        .collect::<Vec<airtable::tags::Tag>>()
        .into_iter();

    for rec in &mut recs {
        let tag = nocodb::tags::Tag {
            id: None,
            name: Some(rec.fields.name.unwrap()),
        };
        db.add_tag(tag).await?;
    }

    let merchants = table
        .get_merchants()
        .await
        .expect("failed to retrieve merchants");
    let mut recs = merchants
        .into_iter()
        .filter(|m| m.id.is_some() && m.fields.name.is_some() && m.fields.purchases.is_some())
        .collect::<Vec<airtable::merchants::Merchant>>()
        .into_iter();

    for rec in &mut recs {
        let merchant = nocodb::merchants::Merchant {
            id: None,
            name: rec.fields.name.unwrap(),
        };
        db.add_merchant(merchant).await?;
    }

    let purchases = table
        .get_purchases()
        .await
        .expect("failed to retrieve purchases");

    for rec in &mut purchases.into_iter() {
        let fields = &rec.fields;

        let tags = fields.tags.unwrap();
        // TODO - is the Vec necessary here?
        let merchant = fields.merchant.unwrap()[0];

        for tag in &mut tags.into_iter() {
            /* TODO
             * - add tags
             * - add merchant
             * - add purchase
             * - associate tags
             * - associate merchant
             */

            // TODO - don't unwrap, use some better matching e.g., `continue`.
            let tag_name = table.get_tag(tag).await?.fields.name.unwrap();
            let tag_id = db.get_tag(&tag_name).await?.id.unwrap();
            // TODO - don't unwrap, use some better matching e.g., `continue`.
            let merchant_name = table.get_merchant(merchant).await?.fields.name.unwrap();
            let merchant_id = db.get_merchant(&merchant_name).await?.id.unwrap();

            let purchase = nocodb::purchases::Purchase {
                id: None,
                amount: fields.amount,
                date: fields.datestr,
                tags: None,
                merchants: None,
            };
            // TODO - avoid unwrap
            let resp = db.add_purchase(purchase).await?;
            db.associate_tag(resp.id.unwrap(), tag_id).await?;
            db.associate_merchant(resp.id.unwrap(), merchant_id).await?;
        }
    }

    Ok(())
}
