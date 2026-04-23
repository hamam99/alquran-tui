use crate::ayah::{AyahResponse, AyahsList};

pub async fn get_ayah_detail(surah_id: i32, list: &mut Vec<AyahsList>) {
    let url = format!("http://api.alquran.cloud/v1/surah/{}", surah_id);
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<AyahResponse>().await {
            Ok(data) => {
                list.clear();
                list.extend(data.data.ayahs);
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
