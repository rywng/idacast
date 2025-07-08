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
    bankara_schedules: ScheduleContainer<MatchNodeBankara>,
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
    #[serde(alias = "regularMatchSetting", alias = "xMatchSetting")]
    match_setting: MatchSetting,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
/// This is different than `MatchNode`, since there are two match settings:
/// Open and Series
///
/// * `start_time`:
/// * `end_time`:
/// * `match_settings`:
struct MatchNodeBankara {
    #[serde(alias = "startTime")]
    start_time: chrono::DateTime<Utc>,
    #[serde(alias = "endTime")]
    end_time: chrono::DateTime<Utc>,
    #[serde(alias = "bankaraMatchSettings")]
    match_settings: Vec<BankaraMatchSetting>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MatchSetting {
    vs_stages: Vec<MatchVsStage>,
    vs_rule: MatchVsRule,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
enum BankaraMode {
    Open,
    Challenge,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct BankaraMatchSetting {
    #[serde(flatten)]
    match_setting: MatchSetting,
    bankara_mode: BankaraMode,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MatchVsStage {
    vs_stage_id: u16,
    name: String,
    id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

    use crate::data::raw_data::{
        BankaraMatchSetting, BankaraMode, MatchNodeBankara, MatchVsRule, RawData, ScheduleContainer
    };

    use super::{MatchNode, MatchSetting, MatchVsStage};

    #[test]
    fn test_deserialize_regular_match() {
        let regular_match_example = r#"
        {"startTime":"2025-07-08T06:00:00Z","endTime":"2025-07-08T08:00:00Z","regularMatchSetting":{"__isVsSetting":"RegularMatchSetting","__typename":"RegularMatchSetting","vsStages":[{"vsStageId":11,"name":"Museum d'Alfonsino","image":{"url":"https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/b9d8cfa186d197a27e075600a107c99d9e21646d116730f0843e0fff0aaba7dd_1.png"},"id":"VnNTdGFnZS0xMQ=="},{"vsStageId":21,"name":"Robo ROM-en","image":{"url":"https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/692365fa7e56cf19cfa403a8546e69cf60fd9ca2171bde66cdaa53dc0e736ac9_1.png"},"id":"VnNTdGFnZS0yMQ=="}],"vsRule":{"name":"Turf War","rule":"TURF_WAR","id":"VnNSdWxlLTA="}},"festMatchSettings":null}
        "#;
        let expedted_regular_match = MatchNode {
            start_time: Utc.with_ymd_and_hms(2025, 7, 8, 6, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 7, 8, 8, 0, 0).unwrap(),
            match_setting: MatchSetting {
                vs_stages: vec![
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
                ],
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

    #[tokio::test]
    async fn test_deserialize_bankara_match() {
        let example_bankara_match = r#"
        {"startTime":"2025-07-08T06:00:00Z","endTime":"2025-07-08T08:00:00Z","bankaraMatchSettings":[{"__isVsSetting":"BankaraMatchSetting","__typename":"BankaraMatchSetting","vsStages":[{"vsStageId":3,"name":"Hagglefish Market","image":{"url":"https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/8dc2f16d39c630bab40cead5b2485ca3559e829d0d3de0c2232c7a62fefb5fa9_1.png"},"id":"VnNTdGFnZS0z"},{"vsStageId":14,"name":"Sturgeon Shipyard","image":{"url":"https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/48684c69d5c5a4ffaf16b712a4895545a8d01196115d514fc878ce99863bb3e9_1.png"},"id":"VnNTdGFnZS0xNA=="}],"vsRule":{"name":"Rainmaker","rule":"GOAL","id":"VnNSdWxlLTM="},"bankaraMode":"CHALLENGE"},{"__isVsSetting":"BankaraMatchSetting","__typename":"BankaraMatchSetting","vsStages":[{"vsStageId":12,"name":"Mahi-Mahi Resort","image":{"url":"https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/8273118c1ffe1bf6fe031c7d8c9795dab52632c9b76e8e9f01f644ac5ae0ccc0_1.png"},"id":"VnNTdGFnZS0xMg=="},{"vsStageId":24,"name":"Lemuria Hub","image":{"url":"https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/2ba481293efc554ac217f21b6d56dd08f9d66e72b286f20714abd5ef1520f47a_1.png"},"id":"VnNTdGFnZS0yNA=="}],"vsRule":{"name":"Tower Control","rule":"LOFT","id":"VnNSdWxlLTI="},"bankaraMode":"OPEN"}],"festMatchSettings":null}
        "#;
        let expected_bankara_match: MatchNodeBankara = MatchNodeBankara {
            start_time: Utc.with_ymd_and_hms(2025, 7, 8, 6, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 7, 8, 8, 0, 0).unwrap(),
            match_settings: vec![
                BankaraMatchSetting {
                    match_setting: MatchSetting {
                        vs_stages: vec![
                            MatchVsStage {
                                vs_stage_id: 3,
                                name: "Hagglefish Market".to_string(),
                                id: "VnNTdGFnZS0z".to_string(),
                            },
                            MatchVsStage {
                                vs_stage_id: 14,
                                name: "Sturgeon Shipyard".to_string(),
                                id: "VnNTdGFnZS0xNA==".to_string(),
                            },
                        ],
                        vs_rule: MatchVsRule {
                            name: "Rainmaker".to_string(),
                            rule: "GOAL".to_string(),
                            id: "VnNSdWxlLTM=".to_string(),
                        },
                    },
                    bankara_mode: BankaraMode::Challenge,
                },
                BankaraMatchSetting {
                    match_setting: MatchSetting {
                        vs_stages: vec![
                            MatchVsStage {
                                vs_stage_id: 12,
                                name: "Mahi-Mahi Resort".to_string(),
                                id: "VnNTdGFnZS0xMg==".to_string(),
                            },
                            MatchVsStage {
                                vs_stage_id: 24,
                                name: "Lemuria Hub".to_string(),
                                id: "VnNTdGFnZS0yNA==".to_string(),
                            },
                        ],
                        vs_rule: MatchVsRule {
                            name: "Tower Control".to_string(),
                            rule: "LOFT".to_string(),
                            id: "VnNSdWxlLTI=".to_string(),
                        },
                    },
                    bankara_mode: BankaraMode::Open,
                },
            ],
        };
        let parsed_bankara_match: MatchNodeBankara =
            serde_json::from_str(example_bankara_match).unwrap();
        assert_eq!(expected_bankara_match, parsed_bankara_match);
    }

    //TODO: Add tests for x rank, and vs stages
    #[tokio::test]
    async fn test_deserialize_x_match() {
        let x_match_example = r#"
                {
          "startTime": "2025-07-08T06:00:00Z",
          "endTime": "2025-07-08T08:00:00Z",
          "xMatchSetting": {
            "__isVsSetting": "XMatchSetting",
            "__typename": "XMatchSetting",
            "vsStages": [
              {
                "vsStageId": 8,
                "name": "Barnacle & Dime",
                "image": {
                  "url": "https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/f70e9f5af477a39ccfab631bfb81c9e2cedb4cd0947fe260847c214a6d23695f_1.png"
                },
                "id": "VnNTdGFnZS04"
              },
              {
                "vsStageId": 19,
                "name": "Crableg Capital",
                "image": {
                  "url": "https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/4e0e9e2046aff1d635e23946d9f0a461486d2aab349079e551037e426ac82c7a_1.png"
                },
                "id": "VnNTdGFnZS0xOQ=="
              }
            ],
            "vsRule": {
              "name": "Splat Zones",
              "rule": "AREA",
              "id": "VnNSdWxlLTE="
            }
          },
          "festMatchSettings": null
        }
        "#;
        let expected_schedule = MatchNode {
            start_time: Utc.with_ymd_and_hms(2025, 7, 8, 6, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 7, 8, 8, 0, 0).unwrap(),
            match_setting: MatchSetting {
                vs_stages: vec![
                    MatchVsStage {
                        vs_stage_id: 8,
                        name: "Barnacle & Dime".to_string(),
                        id: "VnNTdGFnZS04".to_string(),
                    },
                    MatchVsStage {
                        vs_stage_id: 19,
                        name: "Crableg Capital".to_string(),
                        id: "VnNTdGFnZS0xOQ==".to_string(),
                    }
                ],
                vs_rule: MatchVsRule { name: "Splat Zones".to_string(), rule: "AREA".to_string(), id: "VnNSdWxlLTE=".to_string() }
            }
        };
        let parsed_schedule: MatchNode = serde_json::from_str(x_match_example).unwrap();
        assert_eq!(expected_schedule, parsed_schedule);
    }

    #[tokio::test]
    async fn test_parsing_online() {
        let res_text = reqwest::get("https://splatoon3.ink/data/schedules.json").await.unwrap().text().await.unwrap();
        let _parsed: RawData = serde_json::from_str(&res_text).unwrap();
    }
}
