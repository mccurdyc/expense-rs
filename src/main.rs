use std::env;

mod airtable;
use crate::airtable::{tags::Tag, Airtable};

#[tokio::main]
async fn main() {
    let api_token = env::var("AIRTABLE_API_TOKEN").unwrap();
    let base_id = env::var("AIRTABLE_BASE_ID").unwrap();

    if let Ok(table) = Airtable::new(&api_token, &base_id) {
        let tags = table.get_tags().await;
        match tags {
            Ok(tags) => {
                println!(
                    "{:?}",
                    tags.records
                        .into_iter()
                        .filter(|t| t.id.is_some() && t.fields.purchases.is_some())
                        .collect::<Vec<Tag>>()
                );
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

        // let merchants = t.get_merchants().await;
        // match merchants {
        //     Ok(merchants) => {
        //         println!("{:?}", merchants);
        //     }
        //     Err(err) => {
        //         println!("{:?}", err);
        //     }
        // }
        //
        // let purchases = t.get_purchases().await;
        // match purchases {
        //     Ok(purchases) => {
        //         println!("{:?}", purchases);
        //     }
        //     Err(err) => {
        //         println!("{:?}", err);
        //     }
        // }
    }
}
