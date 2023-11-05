/**
* Push notifications to ntfy app
* Subscribe to the alert - jotdown_alerts
*/
#[allow(dead_code)]
const API_URL: &str = "https://ntfy.sh/jotdown_alert";
/**
* Async API call
* send Push notification
* API example https://ntfy.sh
*/
#[allow(dead_code)]
async fn send_notification(msg: &'static str) -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(API_URL).body(msg).send().await?.json::<serde_json::Value>().await?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    // async test
    async fn test_send_msg() {
        let msg: &str = "Jotdown: You have 8 alerts!";
        let response = send_notification(msg).await;
        match response {
            Ok(r) => println!("Push notification sent: {:?}", r),
            Err(_) => println!("Error sending push notification")
        }
    }

}