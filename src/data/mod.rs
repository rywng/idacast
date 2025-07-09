use color_eyre::eyre::Report;
use schedules::{Rule, Schedules, Stage};
use std::fmt::Display;

use color_eyre::Result;
use reqwest::Url;
pub mod raw_data;
pub mod schedules;
pub mod translation;

impl std::error::Error for DataError {}

#[derive(Debug, Clone)]
enum DataError {
    TranslationError(String),
}

impl Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::TranslationError(id) => {
                write!(f, "Failed to find translation to the word with id {}", id)
            }
        }
    }
}
async fn fetch_data() -> Result<raw_data::RawData> {
    let res: String = reqwest::get("https://splatoon3.ink/data/schedules.json")
        .await?
        .text()
        .await?;
    let res: raw_data::RawData = serde_json::from_str(&res)?;

    Ok(res)
}

async fn fetch_translation(lang: String) -> Result<translation::FlattenedTranslationDictionary> {
    let base_url: Url = Url::parse("https://splatoon3.ink/data/locale/")?;
    let joined_url: Url = base_url.join(&lang)?;

    let res: String = reqwest::get(joined_url).await?.text().await?;
    let res: translation::TranslationData = serde_json::from_str(&res)?;
    let res: translation::FlattenedTranslationDictionary = res.into();

    Ok(res)
}

pub fn get_translation(
    mut schedules: Schedules,
    dict: &translation::FlattenedTranslationDictionary,
) -> Result<Schedules> {
    schedules.regular.iter_mut();

    todo!()
}

pub async fn get_schedules(lang: Option<String>) -> Result<schedules::Schedules> {
    let schedules: Schedules = fetch_data().await?.into();

    match lang {
        None => Ok(schedules),
        Some(langcode) => {
            if langcode == "en-US" {
                return Ok(schedules);
            }

            let dict: translation::FlattenedTranslationDictionary =
                fetch_translation(langcode).await?;

            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::{get_schedules, schedules::Schedules};

    use super::{translation::FlattenedTranslationDictionary};

    #[tokio::test]
    async fn test_get_schedules_online() {
        let _schedules: Schedules = get_schedules(None).await.unwrap();
    }
}
