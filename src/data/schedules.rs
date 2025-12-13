use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    raw_data::{self, TimePeriod},
    translation::{Dictionary, Translatable},
};

#[allow(dead_code)] // Very easy to implement, doesn't hurt to keep them.
pub trait Schedule {
    fn get_start_time(&self) -> DateTime<Utc>;
    fn get_end_time(&self) -> DateTime<Utc>;
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Schedules {
    pub regular: Vec<BattleSchedule>,
    pub anarchy_open: Vec<BattleSchedule>,
    pub anarchy_series: Vec<BattleSchedule>,
    pub x_battle: Vec<BattleSchedule>,
    pub work_regular: Vec<CoopSchedule>,
    pub work_big_run: Vec<CoopSchedule>,
    pub work_team_contest: Vec<CoopSchedule>,
    pub league: Vec<LeagueSchedule>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct BattleSchedule {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub stages: Vec<NameID>,
    pub rule: NameID,
}

impl Schedule for BattleSchedule {
    fn get_start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    fn get_end_time(&self) -> DateTime<Utc> {
        self.end_time
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum CoopRule {
    Regular,
    BigRun,
    TeamContest,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CoopSchedule {
    pub rule: CoopRule,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub boss: Option<NameID>,
    pub stage: NameID,
    pub weapons: Vec<NameID>,
}

impl Schedule for CoopSchedule {
    fn get_start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    fn get_end_time(&self) -> DateTime<Utc> {
        self.end_time
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct LeagueSchedule {
    pub event_name: NameID,
    pub desc: String,
    pub details: String,
    pub stages: Vec<NameID>,
    pub rule: NameID,
    pub time_periods: Vec<TimePeriod>,
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
            boss: value.match_setting.boss.as_ref().map(|value| value.into()),
            stage: (&value.match_setting.coop_stage).into(),
            rule: match &value.match_setting.rule {
                Some(rule) => match rule.as_str() {
                    "TEAM_CONTEST" => CoopRule::TeamContest,
                    "BIG_RUN" => CoopRule::BigRun,
                    _ => CoopRule::Regular,
                },
                None => CoopRule::Regular,
            },
            weapons: value
                .match_setting
                .weapons
                .iter()
                .map(|weapon| weapon.into())
                .collect(),
        }
    }
}

impl From<&raw_data::MatchNode> for BattleSchedule {
    fn from(value: &raw_data::MatchNode) -> Self {
        BattleSchedule {
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

impl From<&raw_data::MatchNodeLeague> for LeagueSchedule {
    fn from(value: &raw_data::MatchNodeLeague) -> Self {
        LeagueSchedule {
            event_name: NameID {
                name: value.league_match_setting.league_match_event.name.clone(),
                id: value.league_match_setting.league_match_event.id.clone(),
            },
            desc: value.league_match_setting.league_match_event.desc.clone(),
            details: value
                .league_match_setting
                .league_match_event
                .regulation
                .clone(),
            rule: (&value.league_match_setting.match_setting.vs_rule).into(),
            stages: value
                .league_match_setting
                .match_setting
                .vs_stages
                .iter()
                .map(|stage| stage.into())
                .collect(),
            time_periods: value.time_periods.clone(),
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
                    let schedule_res = BattleSchedule {
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

        value
            .data
            .coop_grouping_schedule
            .big_run_schedules
            .nodes
            .iter()
            .for_each(|schedule| {
                res.work_big_run.push(schedule.into());
            });

        value.data.event_schedules.nodes.iter().for_each(|event| {
            res.league.push(event.into());
        });

        res
    }
}

//TODO: Add tests
