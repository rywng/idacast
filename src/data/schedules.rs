use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    raw_data::{self},
    translation::{Dictionary, Translatable},
};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Schedules {
    pub regular: Vec<Schedule>,
    pub anarchy_open: Vec<Schedule>,
    pub anarchy_series: Vec<Schedule>,
    pub x_battle: Vec<Schedule>,
    pub work_regular: Vec<CoopSchedule>,
    pub work_big_run: Vec<CoopSchedule>,
    pub work_team_contest: Vec<CoopSchedule>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Schedule {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub stages: Vec<NameID>,
    pub rule: NameID,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CoopSchedule {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub boss: NameID,
    pub stage: NameID,
    pub weapons: Vec<NameID>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct NameID {
    pub name: String,
    pub id: String,
}

impl From<&super::raw_data::NameID> for NameID {
    fn from(value: &super::raw_data::NameID) -> Self {
        Self {
            name: value.name.clone(),
            id: value.id.clone(),
        }
    }
}

impl From<super::raw_data::NameID> for NameID {
    fn from(value: super::raw_data::NameID) -> Self {
        Self {
            name: value.name,
            id: value.id,
        }
    }
}

impl Translatable for NameID {
    fn translate(&self, dict: &super::translation::FlattenedTranslationDictionary) -> Self {
        NameID {
            name: dict.lookup(&self.id).unwrap_or(self.name.clone()),
            id: self.id.clone(),
        }
    }
}

impl From<&raw_data::CoopNode> for CoopSchedule {
    fn from(value: &raw_data::CoopNode) -> Self {
        Self {
            start_time: value.start_time,
            end_time: value.end_time,
            boss: (&value.match_setting.boss).into(),
            stage: (&value.match_setting.coop_stage).into(),
            weapons: value
                .match_setting
                .weapons
                .iter()
                .map(|weapon| weapon.into())
                .collect(),
        }
    }
}

impl From<&raw_data::MatchNode> for Schedule {
    fn from(value: &raw_data::MatchNode) -> Self {
        Schedule {
            start_time: value.start_time,
            end_time: value.end_time,
            stages: value
                .match_setting
                .vs_stages
                .iter()
                .map(|stage| stage.into())
                .collect(),
            rule: (&value.match_setting.vs_rule).into(),
        }
    }
}

impl From<raw_data::RawData> for Schedules {
    fn from(value: raw_data::RawData) -> Self {
        let mut res = Self::default();

        value
            .data
            .regular_schedules
            .nodes
            .iter()
            .for_each(|schedule| {
                res.regular.push(schedule.into());
            });

        value.data.x_schedules.nodes.iter().for_each(|schedule| {
            res.x_battle.push(schedule.into());
        });

        value
            .data
            .bankara_schedules
            .nodes
            .iter()
            .for_each(|schedule| {
                for setting in &schedule.match_settings {
                    let schedule_res = Schedule {
                        start_time: schedule.start_time,
                        end_time: schedule.end_time,
                        stages: (setting
                            .match_setting
                            .vs_stages
                            .iter()
                            .map(|stage| stage.into())
                            .collect()),
                        rule: (&setting.match_setting.vs_rule).into(),
                    };
                    match setting.bankara_mode {
                        raw_data::BankaraMode::Open => {
                            res.anarchy_open.push(schedule_res);
                        }
                        raw_data::BankaraMode::Challenge => {
                            res.anarchy_series.push(schedule_res);
                        }
                    }
                }
            });

        value
            .data
            .coop_grouping_schedule
            .regular_schedules
            .nodes
            .iter()
            .for_each(|schedule| {
                res.work_regular.push(schedule.into());
            });
        res
    }
}

//TODO: Add tests
