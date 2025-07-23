use chrono::Local;
use futures::join;
use schedules::{Schedule, Schedules};
use serde_json::Value;
use std::{cmp::min, fmt::Display};
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
    let raw_schedules_fut = fetch_data();

    match lang {
        None => Ok(raw_schedules_fut.await?.into()),
        Some(langcode) => {
            if langcode == "en-US" {
                return Ok(raw_schedules_fut.await?.into());
            }

            let dict_fut = fetch_translation(langcode);
            let (raw_schedules, dict) = join!(raw_schedules_fut, dict_fut);

            let translated: Schedules = translate_schedules(raw_schedules?.into(), &dict?)?;
            Ok(translated)
        }
    }
}

pub fn filter_schedules(schedules: &[Schedule], count: usize) -> Option<&[Schedule]> {
    let mut start: Option<usize> = None;
    let time_now = Local::now();
    for (index, schedule) in schedules.iter().enumerate() {
        if schedule.start_time <= time_now && schedule.end_time >= time_now {
            start = Some(index);
            break;
        }
    }
    match start {
        Some(start) => {
            let end = min(start.saturating_add(count), schedules.len());
            Some(&schedules[start..end])
        }
        None => None,
    }
}

#[cfg(test)]
mod test {
    use chrono::{Duration, Utc};

    use crate::data::{
        fetch_translation, get_schedules, schedules::Schedules,
        translation::FlattenedTranslationDictionary,
    };

    use super::{
        filter_schedules,
        schedules::{Rule, Schedule, Stage},
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
        assert!(!dict.contains_key("non-existent id"));

        let dict: FlattenedTranslationDictionary =
            fetch_translation("ja-JP".to_owned()).await.unwrap();
        assert_eq!(dict.get("VnNTdGFnZS0y").unwrap(), "ゴンズイ地区");
        assert_eq!(dict.get("VnNSdWxlLTM=").unwrap(), "ガチホコバトル");
        assert!(!dict.contains_key("non-existent id"));
    }

    #[tokio::test]
    async fn test_get_schedules_online_with_translation() {
        let _schedules_translated: Schedules =
            get_schedules(Some("zh-CN".to_owned())).await.unwrap();
        dbg!(&_schedules_translated);
    }

    fn get_test_schedule(time_now: chrono::DateTime<Utc>, i: i64) -> Schedule {
        Schedule {
            start_time: time_now - Duration::minutes(90) + Duration::hours(i * 2),
            end_time: time_now + Duration::minutes(30) + Duration::hours(i * 2),
            stages: get_test_stages(i.try_into().unwrap()),
            rule: get_test_rule(),
        }
    }

    fn get_test_stages(start: isize) -> Vec<Stage> {
        let mut sample_stages = Vec::new();
        for i in start..start + 2 {
            sample_stages.push(Stage {
                name: format!("test stage {i}"),
                id: format!("test_{i}"),
            });
        }
        sample_stages
    }

    fn get_test_rule() -> Rule {
        Rule {
            name: "test rule".to_string(),
            id: "test_rule".to_string(),
        }
    }

    #[test]
    fn test_filter_schedule_by_time() {
        let time_now = Utc::now();

        let mut sample_schedules = Vec::new();
        for i in -2..14 {
            // Usually 12 in the api (http://splatoon3.ink/data/schedules.json)
            sample_schedules.push(get_test_schedule(time_now, i));
        }

        let filtered = filter_schedules(&sample_schedules, 3).unwrap();

        assert_eq!(filtered.len(), 3);
        assert_ne!(filtered[0], get_test_schedule(time_now, -1));
        assert_eq!(filtered[0], get_test_schedule(time_now, 0));
        assert_eq!(filtered[1], get_test_schedule(time_now, 1));
        assert_eq!(filtered[2], get_test_schedule(time_now, 2));
        assert_ne!(filtered[2], get_test_schedule(time_now, 3));

        let filtered_alt = filter_schedules(&sample_schedules, usize::MAX).unwrap();
        assert_eq!(filtered_alt.len(), 14)
    }
}
