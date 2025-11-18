/// Raw JSON data fetched from splatoon3.ink
use chrono::{self, Utc};
use serde::{self, Deserialize};

#[derive(Deserialize)]
/// The root of the splatoon3.ink json is a data object, so we need to wrap it.
///
/// * `data`:
pub(super) struct RawData {
    pub data: Data,
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
pub(super) struct Data {
    pub regular_schedules: ScheduleContainer<MatchNode>,
    pub bankara_schedules: ScheduleContainer<MatchNodeBankara>,
    pub x_schedules: ScheduleContainer<MatchNode>,
    pub event_schedules: ScheduleContainer<MatchNodeLeague>,
    // fest_schedules: MatchSchedules,
    pub coop_grouping_schedule: CoopGroupingSchedule,
    // current_fest: MatchSchedules,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CoopGroupingSchedule {
    pub regular_schedules: ScheduleContainer<CoopNode>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(super) struct CoopNode {
    #[serde(alias = "startTime")]
    pub start_time: chrono::DateTime<Utc>,
    #[serde(alias = "endTime")]
    pub end_time: chrono::DateTime<Utc>,
    #[serde(alias = "setting")]
    pub match_setting: CoopSetting,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct CoopSetting {
    pub boss: NameID,
    pub coop_stage: NameID,
    pub weapons: Vec<NameID>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(super) struct NameID {
    pub name: String,
    #[serde(alias = "__splatoon3ink_id")]
    pub id: String,
}

#[derive(Deserialize)]
/// Schedules are usually in a node container with a vector of single schedules.
///
/// * `nodes`:
pub(super) struct ScheduleContainer<T> {
    pub nodes: Vec<T>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(super) struct MatchNode {
    #[serde(alias = "startTime")]
    pub start_time: chrono::DateTime<Utc>,
    #[serde(alias = "endTime")]
    pub end_time: chrono::DateTime<Utc>,
    #[serde(alias = "regularMatchSetting", alias = "xMatchSetting")]
    pub match_setting: MatchSetting,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
/// This is different than `MatchNode`, since there are two match settings:
/// Open and Series
///
/// * `start_time`:
/// * `end_time`:
/// * `match_settings`:
pub(super) struct MatchNodeBankara {
    #[serde(alias = "startTime")]
    pub start_time: chrono::DateTime<Utc>,
    #[serde(alias = "endTime")]
    pub end_time: chrono::DateTime<Utc>,
    #[serde(alias = "bankaraMatchSettings")]
    pub match_settings: Vec<BankaraMatchSetting>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct MatchSetting {
    pub vs_stages: Vec<NameID>,
    pub vs_rule: NameID,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub(super) enum BankaraMode {
    Open,
    Challenge,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct BankaraMatchSetting {
    #[serde(flatten)]
    pub match_setting: MatchSetting,
    pub bankara_mode: BankaraMode,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct MatchNodeLeague {
    pub league_match_setting: LeagueMatchSetting,
    pub time_periods: Vec<TimePeriod>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct LeagueMatchSetting {
    #[serde(flatten)]
    pub match_setting: MatchSetting,
    pub league_match_event: LeagueMatchEvent,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct LeagueMatchEvent {
    id: String,
    name: String,
    desc: String,
    regulation: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct TimePeriod {
    pub start_time: chrono::DateTime<Utc>,
    pub end_time: chrono::DateTime<Utc>,
}

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Utc};

    use crate::data::raw_data::{
        BankaraMatchSetting, BankaraMode, MatchNodeBankara, RawData, TimePeriod,
    };

    use super::{
        CoopNode, CoopSetting, LeagueMatchEvent, LeagueMatchSetting, MatchNode, MatchNodeLeague,
        MatchSetting, NameID,
    };

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
                    NameID {
                        name: "Museum d'Alfonsino".to_string(),
                        id: "VnNTdGFnZS0xMQ==".to_string(),
                    },
                    NameID {
                        name: "Robo ROM-en".to_string(),
                        id: "VnNTdGFnZS0yMQ==".to_string(),
                    },
                ],
                vs_rule: NameID {
                    name: "Turf War".to_string(),
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
                            NameID {
                                name: "Hagglefish Market".to_string(),
                                id: "VnNTdGFnZS0z".to_string(),
                            },
                            NameID {
                                name: "Sturgeon Shipyard".to_string(),
                                id: "VnNTdGFnZS0xNA==".to_string(),
                            },
                        ],
                        vs_rule: NameID {
                            name: "Rainmaker".to_string(),
                            id: "VnNSdWxlLTM=".to_string(),
                        },
                    },
                    bankara_mode: BankaraMode::Challenge,
                },
                BankaraMatchSetting {
                    match_setting: MatchSetting {
                        vs_stages: vec![
                            NameID {
                                name: "Mahi-Mahi Resort".to_string(),
                                id: "VnNTdGFnZS0xMg==".to_string(),
                            },
                            NameID {
                                name: "Lemuria Hub".to_string(),
                                id: "VnNTdGFnZS0yNA==".to_string(),
                            },
                        ],
                        vs_rule: NameID {
                            name: "Tower Control".to_string(),
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
                    NameID {
                        name: "Barnacle & Dime".to_string(),
                        id: "VnNTdGFnZS04".to_string(),
                    },
                    NameID {
                        name: "Crableg Capital".to_string(),
                        id: "VnNTdGFnZS0xOQ==".to_string(),
                    },
                ],
                vs_rule: NameID {
                    name: "Splat Zones".to_string(),
                    id: "VnNSdWxlLTE=".to_string(),
                },
            },
        };
        let parsed_schedule: MatchNode = serde_json::from_str(x_match_example).unwrap();
        assert_eq!(expected_schedule, parsed_schedule);
    }

    #[tokio::test]
    async fn test_parsing_online() {
        let res_text = reqwest::get("https://splatoon3.ink/data/schedules.json")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let _parsed: RawData = serde_json::from_str(&res_text).unwrap();
    }

    #[test]
    fn test_deserialize_coop_regular() {
        let example_schedule = r#"{"startTime":"2025-07-31T00:00:00Z","endTime":"2025-08-01T16:00:00Z","setting":{"__typename":"CoopNormalSetting","boss":{"name":"Cohozuna","id":"Q29vcEVuZW15LTIz"},"coopStage":{"name":"Marooner's Bay","thumbnailImage":{"url":"https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/1a29476c1ab5fdbc813e2df99cd290ce56dfe29755b97f671a7250e5f77f4961_1.png"},"image":{"url":"https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/high_resolution/1a29476c1ab5fdbc813e2df99cd290ce56dfe29755b97f671a7250e5f77f4961_0.png"},"id":"Q29vcFN0YWdlLTY="},"__isCoopSetting":"CoopNormalSetting","weapons":[{"__splatoon3ink_id":"09284d9bd3b61ea1","name":"Dread Wringer","image":{"url":"https://splatoon3.ink/assets/splatnet/v3/weapon_illust/1cf241ee28b282db23d25f1cce3d586151b9b67f4ba20cf5e2e74c82e988c352_0.png"}},{"__splatoon3ink_id":"79e6297eb501599f","name":"Dapple Dualies","image":{"url":"https://splatoon3.ink/assets/splatnet/v3/weapon_illust/f1c8fc32bd90fc9258dc17e9f9bcfd5e6498f6e283709bf1896b78193b8e39e9_0.png"}},{"__splatoon3ink_id":"dde2f92988536cd2","name":"Blaster","image":{"url":"https://splatoon3.ink/assets/splatnet/v3/weapon_illust/29ccca01285a04f42dc15911f3cd1ee940f9ca0e94c75ba07378828afb3165c0_0.png"}},{"__splatoon3ink_id":"c100f88e8b925e1c","name":"Hydra Splatling","image":{"url":"https://splatoon3.ink/assets/splatnet/v3/weapon_illust/34fe0401b6f6a0b09839696fc820ece9570a9d56e3a746b65f0604dec91a9920_0.png"}}]},"__splatoon3ink_king_salmonid_guess":"Cohozuna"}
        "#;

        let expected: CoopNode = CoopNode {
            start_time: Utc.with_ymd_and_hms(2025, 7, 31, 0, 0, 0).unwrap(),
            end_time: Utc.with_ymd_and_hms(2025, 8, 1, 16, 0, 0).unwrap(),
            match_setting: CoopSetting {
                boss: NameID {
                    name: "Cohozuna".to_string(),
                    id: "Q29vcEVuZW15LTIz".to_string(),
                },
                coop_stage: NameID {
                    name: "Marooner's Bay".to_string(),
                    id: "Q29vcFN0YWdlLTY=".to_string(),
                },
                weapons: vec![
                    NameID {
                        name: "Dread Wringer".to_string(),
                        id: "09284d9bd3b61ea1".to_string(),
                    },
                    NameID {
                        name: "Dapple Dualies".to_string(),
                        id: "79e6297eb501599f".to_string(),
                    },
                    NameID {
                        name: "Blaster".to_string(),
                        id: "dde2f92988536cd2".to_string(),
                    },
                    NameID {
                        name: "Hydra Splatling".to_string(),
                        id: "c100f88e8b925e1c".to_string(),
                    },
                ],
            },
        };

        let parsed: CoopNode = serde_json::from_str(&example_schedule).unwrap();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_deserialize_league_match() {
        let example_schedule_node = r#"        { "leagueMatchSetting": { "leagueMatchEvent": { "leagueMatchEventId": "SpecialRush_UltraShot", "name": "Too Many Trizookas!", "desc": "A high-powered Trizooka battle!", "regulationUrl": null, "regulation": "Can you defeat a Trizooka user with a Trizooka of your own?! Now's the time to find out!<br /><br />・ You can only use weapons that come with the Trizooka special.<br />・ The special gauge will fill quickly all by itself!<br />・ Only primary gear abilities will be enabled! Secondary abilities will have no effect.", "id": "TGVhZ3VlTWF0Y2hFdmVudC1TcGVjaWFsUnVzaF9VbHRyYVNob3Q=" }, "vsStages": [ { "vsStageId": 2, "name": "Eeltail Alley", "image": { "url": "https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/898e1ae6c737a9d44552c7c81f9b710676492681525c514eadc68a6780aa52af_1.png" }, "id": "VnNTdGFnZS0y" }, { "vsStageId": 26, "name": "Urchin Underpass", "image": { "url": "https://splatoon3.ink/assets/splatnet/v3/stage_img/icon/low_resolution/249c981fdd888e79ada712ddf899bddbead508d71043af3ff96c90a7b5959c73_1.png" }, "id": "VnNTdGFnZS0yNg==" } ], "__isVsSetting": "LeagueMatchSetting", "__typename": "LeagueMatchSetting", "vsRule": { "name": "Rainmaker", "rule": "GOAL", "id": "VnNSdWxlLTM=" } }, "timePeriods": [ { "startTime": "2025-11-24T02:00:00Z", "endTime": "2025-11-24T04:00:00Z" }, { "startTime": "2025-11-24T06:00:00Z", "endTime": "2025-11-24T08:00:00Z" }  ] }"#;
        let expected: MatchNodeLeague = MatchNodeLeague { league_match_setting: LeagueMatchSetting { match_setting: MatchSetting { vs_stages: vec![
            NameID {
                name: "Eeltail Alley".to_string(),
                id: "VnNTdGFnZS0y".to_string()
            },
            NameID {
                name: "Urchin Underpass".to_string(),
                id: "VnNTdGFnZS0yNg==".to_string()
            }
            ],
            vs_rule: NameID { name: "Rainmaker".to_string(), id: "VnNSdWxlLTM=".to_string() } },
            league_match_event: LeagueMatchEvent{ id: "TGVhZ3VlTWF0Y2hFdmVudC1TcGVjaWFsUnVzaF9VbHRyYVNob3Q=".to_string(), name: "Too Many Trizookas!".to_string(), desc: "A high-powered Trizooka battle!".to_string(), regulation: "Can you defeat a Trizooka user with a Trizooka of your own?! Now's the time to find out!<br /><br />・ You can only use weapons that come with the Trizooka special.<br />・ The special gauge will fill quickly all by itself!<br />・ Only primary gear abilities will be enabled! Secondary abilities will have no effect.".to_string(),},
        },
        time_periods: vec![
            TimePeriod { start_time: Utc.with_ymd_and_hms(2025, 11, 24, 2, 0, 0).unwrap(), end_time: Utc.with_ymd_and_hms(2025, 11, 24, 4, 0, 0).unwrap() },
            TimePeriod{ start_time: Utc.with_ymd_and_hms(2025, 11, 24, 6, 0, 0).unwrap(), end_time: Utc.with_ymd_and_hms(2025, 11, 24, 8, 0, 0).unwrap()}
        ]
    };
        let parsed_schedule: MatchNodeLeague = serde_json::from_str(example_schedule_node).unwrap();
        assert_eq!(parsed_schedule, expected);
    }
}
