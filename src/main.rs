use std::env;

mod airtable;
use crate::airtable::{merchants::Merchant, tags::Tag, Airtable};

#[tokio::main]
async fn main() {
    let api_token = env::var("AIRTABLE_API_TOKEN").unwrap();
    let base_id = env::var("AIRTABLE_BASE_ID").unwrap();

    if let Ok(table) = Airtable::new(&api_token, &base_id) {
        let tags = table.get_tags().await;
        match tags {
            Ok(tags) => {
                let mut recs = tags
                    .records
                    .into_iter()
                    .filter(|t| t.id.is_some() && t.fields.purchases.is_some())
                    .collect::<Vec<Tag>>()
                    .into_iter();

                for rec in &mut recs {
                    println!("{:?}", rec);
                    // TODO - write to nocodb API
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

        //     let merchants = table.get_merchants().await;
        //     match merchants {
        //         Ok(merchants) => {
        //             let mut recs = merchants
        //                 .records
        //                 .into_iter()
        //                 .filter(|m| m.id.is_some() && m.fields.purchases.is_some())
        //                 .collect::<Vec<Merchant>>()
        //                 .into_iter();
        //
        //             for rec in &mut recs {
        //                 println!("{:?}", rec);
        //                 // TODO - write to nocodb SQLite
        //             }
        //         }
        //         Err(err) => {
        //             println!("{:?}", err);
        //         }
        //     }
        //
        //     let purchases = table.get_purchases().await;
        //     match purchases {
        //         Ok(purchases) => {
        //             println!("{:?}", purchases);
        //         }
        //         Err(err) => {
        //             println!("{:?}", err);
        //         }
        //     }
    }
}
