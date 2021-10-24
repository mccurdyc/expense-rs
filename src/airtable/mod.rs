use anyhow::Result;
use reqwest::Client;

pub mod merchants;
pub mod purchases;
pub mod tags;

pub struct Airtable<'a> {
    client: Client,
    api_token: &'a str,
    base_id: &'a str,
}

impl<'a> Airtable<'a> {
    pub fn new(api_token: &'a str, base_id: &'a str) -> Result<Airtable<'a>> {
        Ok(Airtable {
            client: Client::new(),
            api_token,
            base_id,
        })
    }

    fn get_url(&self, table: &str) -> String {
        format!("https://api.airtable.com/v0/{}/{}", self.base_id, table)
    }
}
