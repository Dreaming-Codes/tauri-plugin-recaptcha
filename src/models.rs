use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct DoRecaptchaChallengeRequest {
  pub html: String,
  #[serde(rename = "baseURL")]
  pub base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DoRecaptchaChallengeResponse {
  pub token: String,
}
