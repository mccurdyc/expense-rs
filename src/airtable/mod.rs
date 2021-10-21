use anyhow::Result;
use reqwest::Client;

pub mod purchases;

pub struct Airtable<'a> {
    client: Client,
    api_token: &'a str,
    table_id: &'a str,
}

impl<'a> Airtable<'a> {
    pub fn new(api_token: &'a str, table_id: &'a str) -> Result<Airtable<'a>> {
        Ok(Airtable {
            client: Client::new(),
            api_token,
            table_id,
        })
    }
}
