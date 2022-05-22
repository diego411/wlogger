use std::env;

#[derive(Deserialize, Debug)]
pub struct WEDResponse {
    pub response_code: i32,
    pub is_weeb: bool,
    pub confidence: f32,
    pub number_of_weeb_terms: i32,
}

pub async fn fetch_wed_response(
    channel: String,
    message: String,
) -> Result<WEDResponse, Box<dyn std::error::Error>> {
    let mut req_body = std::collections::HashMap::new();
    req_body.insert("channel", channel);
    req_body.insert("message", message);

    let client = reqwest::Client::new();
    let wed_base_url = env::var("WED_URL").expect("WED URL must be set");
    let resp = client
        .get(wed_base_url + "api/v1/hwis")
        .json(&req_body)
        .send()
        .await?;
    let resp_body = resp.text().await?;
    let response = serde_json::from_str::<WEDResponse>(&resp_body[..])?;
    Ok(response)
}
