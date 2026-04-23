use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AyahResponse {
    pub code: i32,
    pub status: String,
    pub data: AyahDetail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AyahDetail {
    pub number: i32,
    pub name: String,
    pub english_name: String,
    pub english_name_translation: String,
    pub number_of_ayahs: i32,
    pub revelation_type: String,
    pub ayahs: Vec<AyahsList>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AyahsList {
    pub number: i32,
    pub text: String,
    pub number_in_surah: i32,
    pub juz: i32,
    pub manzil: i32,
    pub page: i32,
    pub ruku: i32,
    pub hizb_quarter: i32,
    pub sajda: bool,
}
