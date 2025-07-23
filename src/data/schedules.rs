use chrono::{self, DateTime, Utc};

use super::{
    raw_data,
    translation::{Dictionary, Translatable},
};
#[derive(Debug, Default)]
pub struct Schedules {
    pub regular: Vec<Schedule>,
    pub anarchy_open: Vec<Schedule>,
    pub anarchy_series: Vec<Schedule>,
    pub x_battle: Vec<Schedule>,
}

#[derive(Debug)]
pub struct Schedule {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub stages: Vec<Stage>,
    pub rule: Rule,
}

#[derive(Debug)]
pub struct Stage {
    pub name: String,
    pub id: String,
}

impl Translatable for Stage {
    fn translate(&self, dict: &super::translation::FlattenedTranslationDictionary) -> Self {
        Stage {
            name: dict.lookup(&self.id).unwrap_or(self.name.clone()),
            id: self.id.clone()
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub id: String,
}

impl Translatable for Rule {
    fn translate(&self, dict: &super::translation::FlattenedTranslationDictionary) -> Self {
        Rule {
            name: dict.lookup(&self.id).unwrap_or(self.name.clone()),
            id: self.id.clone()
        }
    }
}

impl From<&raw_data::MatchVsStage> for Stage {
    fn from(value: &raw_data::MatchVsStage) -> Self {
        Stage {
            name: value.name.clone(),
            id: value.id.clone(),
        }
    }
}

impl From<&raw_data::MatchVsRule> for Rule {
    fn from(value: &raw_data::MatchVsRule) -> Self {
        Rule {
            name: value.name.clone(),
            id: value.id.clone(),
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
        let mut res = Schedules {
            regular: vec![],
            anarchy_open: vec![],
            anarchy_series: vec![],
            x_battle: vec![],
        };

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

        res
    }
}

//TODO: Add tests
