use schedules::Schedules;
use serde_json::Value;
use std::fmt::Display;
use translation::Translatable;

use color_eyre::{Report, Result, eyre::Ok};
use reqwest::Url;
pub mod raw_data;
pub mod schedules;
pub mod translation;

impl std::error::Error for DataError {}

#[derive(Debug, Clone)]
enum DataError {
    ObjectNonExistError(String),
    TranslationError(String),
}

impl Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::TranslationError(id) => {
                write!(f, "Failed to find translation to the word with id {}", id)
            }
            DataError::ObjectNonExistError(object) => {
                write!(f, "Object {} should exist in the data", object)
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
    let joined_url: Url = base_url.join(&format!("{}.json", lang))?;

    let res: String = reqwest::get(joined_url).await?.text().await?;

    // Need to sanitize data, workaround for https://github.com/misenhower/splatoon3.ink/issues/94
    let mut res: Value = serde_json::from_str(&res)?;
    let rules = match res.get_mut("rules") {
        Some(rules) => rules,
        None => {
            return Err(Report::new(DataError::ObjectNonExistError(
                "rules".to_string(),
            )));
        }
    };
    match rules.as_object_mut() {
        Some(rule_obj) => {
            rule_obj.remove("undefined");
        }
        None => {
            return Err(Report::new(DataError::ObjectNonExistError(
                "rules".to_string(),
            )));
        }
    }

    // After sanitization, continue parsing
    let res: translation::TranslationData = serde_json::from_value(res)?;
    let res: translation::FlattenedTranslationDictionary = res.into();

    Ok(res)
}

pub fn translate_schedules(
    mut schedules: Schedules,
    dict: &translation::FlattenedTranslationDictionary,
) -> Result<Schedules> {
    for schedule in &mut schedules.regular {
        translate_schedule(dict, schedule);
    }
    schedules
        .regular
        .iter_mut()
        .for_each(|schedule| translate_schedule(dict, schedule));

    schedules
        .anarchy_open
        .iter_mut()
        .for_each(|schedule| translate_schedule(dict, schedule));

    schedules
        .anarchy_series
        .iter_mut()
        .for_each(|schedule| translate_schedule(dict, schedule));

    schedules
        .x_battle
        .iter_mut()
        .for_each(|schedule| translate_schedule(dict, schedule));

    Ok(schedules)
}

fn translate_schedule(
    dict: &std::collections::HashMap<String, String>,
    schedule: &mut schedules::Schedule,
) {
    for stage in &mut schedule.stages {
        *stage = stage.translate(dict);
    }
    schedule.rule = schedule.rule.translate(dict);
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

            let translated: Schedules = translate_schedules(schedules, &dict)?;
            Ok(translated)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::{
        fetch_translation, get_schedules, schedules::Schedules,
        translation::FlattenedTranslationDictionary,
    };

    #[tokio::test]
    async fn test_get_schedules_online() {
        let _schedules: Schedules = get_schedules(None).await.unwrap();
        dbg!(&_schedules);
    }

    #[tokio::test]
    async fn test_get_dictionary_online() {
        let dict: FlattenedTranslationDictionary =
            fetch_translation("zh-CN".to_owned()).await.unwrap();
        assert_eq!(dict.get("VnNTdGFnZS0y").unwrap(), "鳗鲶区");
        assert_eq!(dict.get("VnNSdWxlLTM=").unwrap(), "真格鱼虎对战");
        assert!(dict.get("non-existent id").is_none());

        let dict: FlattenedTranslationDictionary =
            fetch_translation("ja-JP".to_owned()).await.unwrap();
        assert_eq!(dict.get("VnNTdGFnZS0y").unwrap(), "ゴンズイ地区");
        assert_eq!(dict.get("VnNSdWxlLTM=").unwrap(), "ガチホコバトル");
        assert!(dict.get("non-existent id").is_none());
    }

    #[tokio::test]
    async fn test_get_schedules_online_with_translation() {
        let _schedules_translated: Schedules =
            get_schedules(Some("zh-CN".to_owned())).await.unwrap();
        dbg!(&_schedules_translated);
    }
}
