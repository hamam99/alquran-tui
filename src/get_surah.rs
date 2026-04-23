use crate::surah::{SurahDetail, SurahResponse};

pub async fn get_surah(list: &mut Vec<SurahDetail>) {
    match reqwest::get("http://api.alquran.cloud/v1/surah").await {
        Ok(resp) => match resp.json::<SurahResponse>().await {
            Ok(data) => {
                list.clear();
                list.extend(data.data);
            }
            Err(e) => {
                // eprintln!("JSON error: {}", e);
            }
        },
        Err(e) => {
            // eprintln!("Request error: {}", e);
        }
    }
}
