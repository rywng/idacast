use color_eyre::eyre::Report;
use std::fmt::Display;

use color_eyre::Result;
use reqwest::Url;
pub mod raw_data;
pub mod schedules;
pub mod translation_data;

#[derive(Debug, Clone)]
enum DataError {
    TranslationError(String),
}

impl Display for crate::data::DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::TranslationError(id) => {
                write!(f, "Failed to find translation to the word with id {}", id)
            }
        }
    }
}

impl std::error::Error for DataError {}

async fn fetch_data() -> Result<raw_data::RawData> {
    let res: String = reqwest::get("https://splatoon3.ink/data/schedules.json")
        .await?
        .text()
        .await?;
    let res: raw_data::RawData = serde_json::from_str(&res)?;

    Ok(res)
}

async fn fetch_translation(
    lang: String,
) -> Result<translation_data::FlattenedTranslationDictionary> {
    let base_url: Url = Url::parse("https://splatoon3.ink/data/locale/")?;
    let joined_url: Url = base_url.join(&lang)?;

    let res: String = reqwest::get(joined_url).await?.text().await?;
    let res: translation_data::TranslationData = serde_json::from_str(&res)?;
    let res: translation_data::FlattenedTranslationDictionary = res.into();

    Ok(res)
}

fn lookup_translation(
    id: String,
    dictionary: &translation_data::FlattenedTranslationDictionary,
) -> Result<String> {
    dictionary
        .get(&id)
        .ok_or_else(|| Report::new(DataError::TranslationError(id)))
        .cloned()
}

#[cfg(test)]
mod test {
    use super::{lookup_translation, translation_data::FlattenedTranslationDictionary};

    #[test]
    fn test_translation_lookup_with_dictionary() {
        let dict = FlattenedTranslationDictionary::from([
            ("VnNTdGFnZS0x".to_string(), "温泉花大峡谷".to_string()),
            ("VnNTdGFnZS0y".to_string(), "鳗鲶区".to_string()),
        ]);

        assert_eq!(
            lookup_translation("VnNTdGFnZS0x".to_string(), &dict).unwrap(),
            "温泉花大峡谷".to_string()
        );

        assert!(lookup_translation("nonexistent".to_string(), &dict).is_err());
    }
}
