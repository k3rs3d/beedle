use reqwest::Client;
use serde_json::json;

pub async fn process_payment(total_amount: i32, payment_token: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let response = client.post("https://api.stripe.com/v1/charges")
        .header("Authorization", format!("Bearer {}", "your_api_key"))
        .form(&json!({
            "amount": total_amount,
            "currency": "usd",
            "source": payment_token,
        }))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}