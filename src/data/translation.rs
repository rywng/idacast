use serde::Deserialize;
use std::collections::HashMap;

use super::DataError;

pub(super) trait Translatable {
    fn translate(&self, dict: &FlattenedTranslationDictionary) -> Self;
}

pub(super) trait Dictionary {
    fn lookup(&self, id: &str) -> Result<String, DataError>;
}

type TranslationMap = HashMap<String, TranslationContent>;

pub type FlattenedTranslationDictionary = HashMap<String, String>;

impl Dictionary for FlattenedTranslationDictionary {
    fn lookup(&self, id: &str) -> Result<String, DataError> {
        self.get(id)
            .ok_or_else(|| DataError::TranslationError(id.to_string()))
            .cloned()
    }
}

impl From<TranslationData> for FlattenedTranslationDictionary {
    fn from(value: TranslationData) -> Self {
        let mut res: HashMap<String, String> = HashMap::new();
        for translation_map in [value.bosses, value.stages, value.rules] {
            translation_map
                .iter()
                .for_each(|(id, translation_content)| {
                    res.insert(id.to_string(), translation_content.name.clone());
                })
        }

        res
    }
}

#[derive(Deserialize, Debug)]
pub struct TranslationData {
    // gear: TranslationMap,
    stages: TranslationMap,
    rules: TranslationMap,
    // weapons: TranslationMap,
    bosses: TranslationMap,
    // brands: TranslationMap,
    // powers: TranslationMap,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TranslationContent {
    name: String,
}

#[cfg(test)]
mod test {
    use crate::data::translation::Dictionary;
    use crate::data::translation::{FlattenedTranslationDictionary, TranslationContent};

    use super::TranslationMap;

    #[test]
    fn test_translation_map() {
        let example_map_data = r#"
        {
            "VnNTdGFnZS0x": {
              "name": "温泉花大峡谷"
            },
            "VnNTdGFnZS0y": {
              "name": "鳗鲶区"
            },
            "VnNTdGFnZS0z": {
              "name": "烟管鱼市场"
            },
            "VnNTdGFnZS00": {
              "name": "竹蛏疏洪道"
            },
            "VnNTdGFnZS02": {
              "name": "鱼肉碎金属"
            },
            "VnNTdGFnZS0xMA==": {
              "name": "真鲭跨海大桥"
            },
            "VnNTdGFnZS0xMQ==": {
              "name": "金眼鲷美术馆"
            }
        }"#;
        let result: TranslationMap = serde_json::from_str(example_map_data).unwrap();
        assert_eq!(
            result["VnNTdGFnZS02"],
            TranslationContent {
                name: "鱼肉碎金属".to_string()
            }
        );
        assert_eq!(
            result["VnNTdGFnZS0xMA=="],
            TranslationContent {
                name: "真鲭跨海大桥".to_string()
            }
        );
        assert!(result.get("non-existent-id").is_none())
    }

    #[test]
    fn test_translation_lookup_with_dictionary() {
        let dict: FlattenedTranslationDictionary = FlattenedTranslationDictionary::from([
            ("VnNTdGFnZS0x".to_string(), "温泉花大峡谷".to_string()),
            ("VnNTdGFnZS0y".to_string(), "鳗鲶区".to_string()),
        ]);

        assert_eq!(
            dict.lookup("VnNTdGFnZS0x").unwrap(),
            "温泉花大峡谷".to_string()
        );

        assert!(dict.lookup("nonexistent").is_err());
    }
}
