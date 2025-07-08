/// Raw JSON data fetched from splatoon3.ink
use chrono::{self, Utc};
use serde::{self, Deserialize};

#[derive(Deserialize)]
/// The root of the splatoon3.ink json is a data object, so we need to wrap it.
///
/// * `data`: 
struct RawData {
    data: Data,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
/// Data for stages, currently only supports regular, ranked and X.
/// TODO: event, fest, and grizzco may be supported in the future.
///
/// * `regular_schedules`:
/// * `bankara_schedules`:
/// * `x_schedules`:
/// * `vs_stages`:
pub struct Data {
    regular_schedules: ScheduleContainer<MatchNode>,
    bankara_schedules: ScheduleContainer<MatchNode>,
    x_schedules: ScheduleContainer<MatchNode>,
    // event_schedules: MatchSchedules,
    // fest_schedules: MatchSchedules,
    // coop_grouping_schedule: MatchSchedules,
    // current_fest: MatchSchedules,
    vs_stages: VsStages,
}

#[derive(Deserialize)]
/// Schedules are usually in a node container with a vector of single schedules.
///
/// * `nodes`:
struct ScheduleContainer<T> {
    nodes: Vec<T>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct MatchNode {
    #[serde(alias = "startTime")]
    start_time: chrono::DateTime<Utc>,
    #[serde(alias = "endTime")]
    end_time: chrono::DateTime<Utc>,
    #[serde(alias = "regularMatchSetting", alias = "bankaraMatchSettings")]
    match_setting: MatchSetting,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MatchSetting {
    vs_stages: (MatchVsStage, MatchVsStage),
    vs_rule: MatchVsRule,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MatchVsStage {
    vs_stage_id: u16,
    name: String,
    id: String,
}

#[derive(Deserialize)]
struct VsStages {
    nodes: Vec<MatchVsStage>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct MatchVsRule {
    name: String,
    rule: String,
    id: String,
}

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Utc};

    use crate::data::raw_data::MatchVsRule;

    use super::{MatchNode, MatchSetting, MatchVsStage};

    #[test]
    fn test_deserialize_regular_match() {
        let regular_match_example = r#"
                {
          "startTime": "2025-07-08T06:00:00Z",
          "endTime": "2025-07-08T08:00:00Z",
          "regularMatchSetting": {
            "__isVsSetting": "RegularMatchSetting",
            "__typename": "RegularMatchSetting",
            "vsStages": [
              {
                "vsStageId": 11,
                "name": "Museum d'Alfonsino",
                "image": {
                  "url": "https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/b9d8cfa186d197a27e075600a107c99d9e21646d116730f0843e0fff0aaba7dd_1.png"
                },
                "id": "VnNTdGFnZS0xMQ=="
              },
              {
                "vsStageId": 21,
                "name": "Robo ROM-en",
                "image": {
                  "url": "https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/692365fa7e56cf19cfa403a8546e69cf60fd9ca2171bde66cdaa53dc0e736ac9_1.png"
                },
                "id": "VnNTdGFnZS0yMQ=="
              }
            ],
            "vsRule": {
              "name": "Turf War",
              "rule": "TURF_WAR",
              "id": "VnNSdWxlLTA="
            }
          },
          "festMatchSettings": null
        }
        "#;
        let expedted_regular_match = MatchNode {
            start_time: Utc.with_ymd_and_hms(2025, 7, 8, 6, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 7, 8, 8, 0, 0).unwrap(),
            match_setting: MatchSetting {
                vs_stages: (
                    MatchVsStage {
                        vs_stage_id: 11,
                        name: "Museum d'Alfonsino".to_string(),
                        id: "VnNTdGFnZS0xMQ==".to_string(),
                    },
                    MatchVsStage {
                        vs_stage_id: 21,
                        name: "Robo ROM-en".to_string(),
                        id: "VnNTdGFnZS0yMQ==".to_string(),
                    },
                ),
                vs_rule: MatchVsRule {
                    name: "Turf War".to_string(),
                    rule: "TURF_WAR".to_string(),
                    id: "VnNSdWxlLTA=".to_string(),
                },
            },
        };

        let parsed_regular_match: MatchNode = serde_json::from_str(regular_match_example).unwrap();
        assert_eq!(parsed_regular_match, expedted_regular_match);
    }
}
