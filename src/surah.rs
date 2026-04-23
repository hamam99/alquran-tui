use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SurahResponse {
    pub code: i32,
    pub status: String,
    pub data: Vec<SurahDetail>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SurahDetail {
    pub number: i32,
    pub name: String,
    pub english_name: String,
    pub english_name_translation: String,
    pub number_of_ayahs: i32,
    pub revelation_type: String,
}
