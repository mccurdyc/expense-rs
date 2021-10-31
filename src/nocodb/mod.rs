use anyhow::Result;
use reqwest::Client;

pub mod merchants;
pub mod purchases;
pub mod tags;

pub struct NocoDB<'a> {
    client: Client,
    host: &'a str,
    project_id: &'a str,
    api_token: &'a str,
}

impl<'a> NocoDB<'a> {
    pub fn new(host: &'a str, project_id: &'a str, api_token: &'a str) -> Result<NocoDB<'a>> {
        Ok(NocoDB {
            client: Client::new(),
            project_id,
            host,
            api_token,
        })
    }

    fn get_url(&self, path: &str) -> String {
        format!("{}/nc/{}/api/v1/{}", self.host, self.project_id, path)
    }
}
