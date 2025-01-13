use serde_json::Value;

pub async fn response_to_value(
    res: reqwest::Result<reqwest::Response>,
) -> leptos::error::Result<Value, String> {
    let Ok(res) = res else {
        return Err("unknown error".into());
    };

    if !res.status().is_success() {
        let error_message = res
            .json::<Value>()
            .await
            .ok()
            .and_then(|json| {
                json.get("error")
                    .and_then(|msg| msg.as_str().map(String::from))
            })
            .unwrap_or_else(|| "Unknown error".to_string());
        Err(error_message)
    } else {
        let signup_res = res
            .bytes()
            .await
            .ok()
            .map(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
            .flatten();

        match signup_res {
            Some(login_res) => Ok(login_res),
            None => Err("unknown error".to_string()), // Response is not valid json
        }
    }
}
