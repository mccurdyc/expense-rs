use std::env;

mod airtable;
use crate::airtable::Airtable;

#[tokio::main]
async fn main() {
    let api_token = env::var("AIRTABLE_API_TOKEN").unwrap();
    let table_id = env::var("AIRTABLE_BASE_ID").unwrap();

    if let Some(t) = Airtable::new(&api_token, &table_id).ok() {
        if let Some(ps) = t.get_purchases().await.ok() {
            println!("{:?}", ps);
        }
    }
}
