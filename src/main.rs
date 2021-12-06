use anyhow::Error;
use std::{env, process::exit};

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

    for rec in purchases.into_iter() {
        println!("rec: {:?}", rec);
        let purchase = nocodb::purchases::Purchase {
            id: None,
            amount: rec.fields.amount,
            date: rec.fields.datestr,
        };
        let resp = db.add_purchase(purchase).await?;
        println!("purchase_id: {:?}", resp.id);

        if let Some(purchase_id) = resp.id {
            // https://docs.nocodb.com/setup-and-usages/link-to-another-record#relationship-types
            // For every many to many relation defined between tables, NocoDB augments many to many
            // relationship column in the other table automatically.
            let merchant = rec.fields.merchant.unwrap();
            let merchant_name = table.get_merchant(&merchant[0]).await?.fields.name.unwrap();
            let merchant_id = db.get_merchant(&merchant_name).await?.id.unwrap();
            println!(
                "associating merchant: purchase: {:?}, merchant: {:?}",
                purchase_id, merchant_id
            );
            if let Err(e) = db.associate_merchant(purchase_id, merchant_id).await {
                eprintln!("associate_merchant error: {:?}", e);
                exit(1);
            }

            let tags = rec.fields.tags.unwrap();
            println!("tags: purchase: {:?}, tags: {:?}", purchase_id, tags);
            for tag in tags.iter() {
                let tag_name = table.get_tag(tag).await?.fields.name.unwrap();
                println!(
                    "tag name: purchase: {:?}, name: {:?}",
                    purchase_id, tag_name
                );
                let tag_id = db.get_tag(&tag_name).await?.id.unwrap();
                println!(
                    "associating tag: purchase: {:?}, tag: {:?}",
                    purchase_id, tag_id
                );
                if let Err(e) = db.associate_tag(purchase_id, tag_id).await {
                    eprintln!("associate_tag error: {:?}", e);
                    exit(1);
                }
            }
        } else {
            eprintln!("failed with response: {:?}", resp);
            exit(1);
        }
    }

    Ok(())
}
