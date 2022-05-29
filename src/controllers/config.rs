use std::env;

#[derive(Deserialize, Debug)]
pub struct ConfigResponse {
    pub channels: std::collections::HashSet<String>,
}

pub async fn fetch_config() -> Result<ConfigResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let config_service_url = env::var("CONFIG_SERVER_URL").expect("CONFIG_SERVER_URL must be set");
    let resp = client
        .get(config_service_url + "api/v1/config")
        .send()
        .await?;
    let resp_body = resp.text().await?;
    let response = serde_json::from_str::<ConfigResponse>(&resp_body[..])?;
    Ok(response)
}
