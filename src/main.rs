use std::env;

mod airtable;
use crate::airtable::Airtable;

#[tokio::main]
async fn main() {
    let api_token = env::var("AIRTABLE_API_TOKEN").unwrap();
    let base_id = env::var("AIRTABLE_BASE_ID").unwrap();

    if let Ok(t) = Airtable::new(&api_token, &base_id) {
        let tags = t.get_tags().await;
        match tags {
            Ok(tags) => {
                println!("{:?}", tags);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

        let merchants = t.get_merchants().await;
        match merchants {
            Ok(merchants) => {
                println!("{:?}", merchants);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

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
