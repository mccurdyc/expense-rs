use anyhow::Error;
use std::env;

mod airtable;
mod nocodb;

use crate::airtable::Airtable;
use crate::nocodb::NocoDB;

// TODO:
// - rate limit / paginate Airtable requests.
// - implement nocodb module

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
        .filter(|t| t.id.is_some() && t.fields.purchases.is_some())
        .collect::<Vec<airtable::tags::Tag>>()
        .into_iter();

    for rec in &mut recs {
        let tag = nocodb::tags::Tag {
            name: &rec.fields.name,
        };
        db.add_tag(tag).await?;
    }

    // TODO - merchants
    // TODO - purchases - will have to query Airtable to get merchant and tag names from IDs

    Ok(())
}
