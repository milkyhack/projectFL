use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Localization {
    pub current_lang: String,
    client: Arc<Client>,
}

impl Localization {
    pub fn new() -> Self {
        Localization {
            current_lang: "en".to_string(),
            client: Arc::new(Client::new()),
        }
    }

    pub fn set_language(&mut self, lang: &str) {
        self.current_lang = lang.to_string();
    }

    pub async fn translate(&self, text: &str) -> String {
        if self.current_lang == "en" {
            return text.to_string();
        }

        let response = self
            .client
            .post("https://libretranslate.de/translate")
            .json(&serde_json::json!({
                "q": text,
                "source": "en",
                "target": self.current_lang,
                "format": "text"
            }))
            .send()
            .await;

        match response {
            Ok(resp) => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    json["translatedText"]
                        .as_str()
                        .unwrap_or(text)
                        .to_string()
                } else {
                    text.to_string()
                }
            }
            Err(_) => text.to_string(),
        }
    }
}
